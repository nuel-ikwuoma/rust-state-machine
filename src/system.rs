use core::ops::AddAssign;
use num::{traits::CheckedAdd, One, Zero};
use std::collections::BTreeMap;

pub trait Config {
	type AccountID: Ord + Clone;
	type BlockNumber: CheckedAdd + One + Zero + Copy + AddAssign;
	type Nonce: CheckedAdd + One + Zero + Copy;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// The current block number.
	block_number: T::BlockNumber,
	/// A map from an account to their nonce.
	nonce: BTreeMap<T::AccountID, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += T::BlockNumber::one();
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: T::AccountID) {
		let prev_nonce = *self.nonce.get(&who).unwrap_or(&T::Nonce::one());
		let new_nonce = prev_nonce + T::Nonce::one();
		self.nonce.insert(who, new_nonce);
	}

	pub fn get_nonce(&self, who: T::AccountID) -> T::Nonce {
		*self.nonce.get(&who).unwrap_or(&T::Nonce::zero())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type AccountID = String;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn init_block_number() {
		let new_system = super::Pallet::<TestConfig>::new();
		assert_eq!(new_system.block_number(), 0);
	}

	#[test]
	fn next_block_number() {
		let mut new_system = super::Pallet::<TestConfig>::new();

		assert_eq!(new_system.block_number(), 0);
		new_system.inc_block_number();
		assert_eq!(new_system.block_number(), 1);

		let mut i = 5;

		while i > 0 {
			new_system.inc_block_number();
			i = i - 1;
		}

		assert_eq!(new_system.block_number(), 6);
	}

	#[test]
	fn account_nonce() {
		let mut new_system = super::Pallet::<TestConfig>::new();

		new_system.inc_nonce("alice".to_string());

		assert_eq!(new_system.get_nonce("alice".to_string()), 1);
		assert_eq!(new_system.get_nonce("bob".to_string()), 0);
	}

	#[test]
	fn init_system() {
		let mut new_system = super::Pallet::<TestConfig>::new();
		let alice = String::from("alice");
		new_system.inc_block_number();
		new_system.inc_nonce(alice.clone());

		assert_eq!(new_system.block_number(), 1);
		assert_eq!(new_system.get_nonce(alice), 1);
	}
}
