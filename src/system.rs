use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet {
	/// The current block number.
	block_number: u32,
	/// A map from an account to their nonce.
	nonce: BTreeMap<String, u32>,
}

impl Pallet {
	/// Create a new instance of the System Pallet.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
		Self { block_number: 0, nonce: BTreeMap::new() }
	}

	/// Get the current block number.
	pub fn block_number(&self) -> u32 {
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number += 1;
	}

	// Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: String) {
		let prev_nonce = *self.nonce.get(&who).unwrap_or(&0);
		self.nonce.insert(who, prev_nonce + 1);
	}

	pub fn get_nonce(&self, who: String) -> u32 {
		*self.nonce.get(&who).unwrap_or(&0)
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn init_block_number() {
		let new_system = super::Pallet::new();
		assert_eq!(new_system.block_number(), 0);
	}

	#[test]
	fn next_block_number() {
		let mut new_system = super::Pallet::new();

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
		let mut new_system = super::Pallet::new();

		new_system.inc_nonce("alice".to_string());

		assert_eq!(new_system.get_nonce("alice".to_string()), 1);
		assert_eq!(new_system.get_nonce("bob".to_string()), 0);
	}

    #[test]
    fn init_system() {
        let mut new_system = super::Pallet::new();
        let alice = String::from("alice");
        new_system.inc_block_number();
        new_system.inc_nonce(alice.clone());

        assert_eq!(new_system.block_number(), 1);
        assert_eq!(new_system.get_nonce(alice), 1);
    }
}
