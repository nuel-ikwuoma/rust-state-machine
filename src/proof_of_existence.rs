use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	/// The type which represents the content that can be claimed using this pallet.
	/// Could be the content directly as bytes, or better yet the hash of that content.
	/// We leave that decision to the runtime developer.
	type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
	claims: BTreeMap<T::Content, T::AccountID>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	/// Get the owner (if any) of a claim.
	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountID> {
		self.claims.get(claim)
	}

	/// Create a new claim on behalf of the `caller`.
	/// This function will return an error if someone already has claimed that content.
	pub fn create_claim(&mut self, caller: T::AccountID, claim: T::Content) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err(&"this content is already claimed");
		}
		let _res = self.claims.insert(claim, caller);
		Ok(())
	}

	/// Revoke an existing claim on some content.
	/// This function should only succeed if the caller is the owner of an existing claim.
	/// It will return an error if the claim does not exist, or if the caller is not the owner.
	pub fn revoke_claim(&mut self, caller: T::AccountID, claim: T::Content) -> DispatchResult {
		let owner = self.get_claim(&claim).ok_or("No owner for claim")?;
		if *owner == caller {
			self.claims.remove(&claim);
		} else {
			return Err("caller is not `claim` owner");
		}
		Ok(())
	}
}

// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
pub enum Call<T: Config> {
	CreateClaim { claim: T::Content },
	RevokeClaim { claim: T::Content },
}

/// Implementation of the dispatch logic, mapping from `POECall` to the appropriate underlying
/// function we want to execute.
impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::AccountID;
	type Call = Call<T>;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
		match call {
			Call::CreateClaim { claim } => self.create_claim(caller, claim),
			Call::RevokeClaim { claim } => self.revoke_claim(caller, claim),
		}
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountID = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let mut poe = super::Pallet::<TestConfig>::new();
		let claim = "0xcontent claim";
		let alice = "alice";
		let bob = "bob";

		assert_eq!(poe.get_claim(&"non-existent claim"), None);

		let _res = poe.create_claim(alice, claim);
		assert_eq!(poe.get_claim(&claim), Some(&alice));

		assert_eq!(poe.create_claim(&bob, &claim), Err("this content is already claimed"));
		assert_eq!(poe.revoke_claim(&alice, &claim), Ok(()));
		assert_eq!(poe.create_claim(&bob, &claim), Ok(()));
	}
}
