use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

use crate::support;

pub trait Config: crate::system::Config {
	type Balance: Zero + CheckedAdd + CheckedSub + Copy;
}

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	// A simple storage mapping from accounts (`String`) to their balances (`u128`).
	balances: BTreeMap<T::AccountID, T::Balance>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the balances module.
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	/// Set the balance of an account `who` to some `amount`.
	pub fn set_balance(&mut self, who: &T::AccountID, amt: T::Balance) {
		self.balances.insert(who.clone(), amt);
	}

	/// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.
	pub fn balance(&self, who: &T::AccountID) -> T::Balance {
		*self.balances.get(who).unwrap_or(&T::Balance::zero())
	}

	/// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	pub fn transfer(
		&mut self,
		caller: T::AccountID,
		to: T::AccountID,
		amount: T::Balance,
	) -> support::DispatchResult {
		let init_balance_caller = self.balance(&caller);
		let init_balance_to = self.balance(&to);

		let final_balance_caller =
			init_balance_caller.checked_sub(&amount).ok_or("Not enough funds")?;
		let final_balance_to =
			init_balance_to.checked_add(&amount).ok_or("Balance overflow error")?;

		self.set_balance(&caller, final_balance_caller);
		self.set_balance(&to, final_balance_to);

		Ok(())
	}
}

// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
pub enum Call<T: Config> {
	Transfer { to: T::AccountID, amount: T::Balance },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
	type Caller = T::AccountID;
	type Call = Call<T>;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> support::DispatchResult {
		match call {
			Call::Transfer { to, amount } => self.transfer(caller, to, amount)?,
		}
		Ok(())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl crate::system::Config for TestConfig {
		type AccountID = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	impl super::Config for TestConfig {
		type Balance = u128;
	}

	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::<TestConfig>::new();

		assert_eq!(balances.balance(&"alice".to_string()), 0);
		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.balance(&"alice".to_string()), 100);
		assert_eq!(balances.balance(&"bob".to_string()), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::<TestConfig>::new();

		assert!(balances.transfer("alice".to_string(), "bob".to_string(), 100).is_err());

		balances.set_balance(&"alice".to_string(), 100);
		assert!(balances.transfer("alice".to_string(), "bob".to_string(), 50).is_ok());

		assert_eq!(balances.balance(&"alice".to_string()), 50);
		assert_eq!(balances.balance(&"bob".to_string()), 50);
	}
}
