mod balances;
mod proof_of_existence;
mod support;
mod system;

use support::Dispatch;

mod types {
	use crate::support;

	pub type AccountID = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = support::Extrinsic<AccountID, crate::RuntimeCall>;
	pub type Header = support::Header<BlockNumber>;
	pub type Block = support::Block<Header, Extrinsic>;
	pub type Content = &'static str;
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
	proof_of_existence: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountID = types::AccountID;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
	type Content = types::Content;
}

fn main() {
	// instantiate the runtime
	let mut runtime = Runtime::new();

	// set alice's balance to 100 tokens
	let alice = String::from("alice");
	let bob = String::from("bob");
	let charlie: String = String::from("charlie");

	runtime.balances.set_balance(&alice, 100);

	let block_1 = types::Block {
		header: types::Header { block_number: 1 },
		extrinsics: vec![
			types::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer {
					to: bob.clone(),
					amount: 50,
				}),
			},
			types::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::balances(balances::Call::transfer {
					to: charlie.clone(),
					amount: 20,
				}),
			},
		],
	};

	let block_2 = types::Block {
		header: types::Header { block_number: 2 },
		extrinsics: vec![
			types::Extrinsic {
				caller: charlie.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
					claim: "0xContent claim",
				}),
			},
			types::Extrinsic {
				caller: bob.clone(),
				call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
					claim: "0xContent claim",
				}),
			},
		],
	};

	runtime.execute_block(block_1).expect("Invalid block");
	runtime.execute_block(block_2).expect("Invalid block");

	// uncomment lines to cause panic.

	// let block_3 = types::Block {
	// 	header: types::Header{block_number: 2},
	// 	extrinsics: vec!{},
	// };
	// runtime.execute_block(block_3).expect("Invalid block");

	println!("{:#?}", runtime);
}
