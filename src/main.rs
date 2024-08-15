mod balances;
mod support;
mod system;

use support::{Dispatch, DispatchResult};

mod types {
	use crate::support;

	pub type AccountId = String;
	pub type Balance = u128;
	pub type BlockNumber = u32;
	pub type Nonce = u32;
	pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
	pub type Header = support::Header<BlockNumber>;
	pub type Block = support::Block<Header, Extrinsic>;
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
	// BalancesTransfer { to: types::AccountId, amount: types::Balance },
	Balances(balances::Call<Runtime>),
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<Self>,
	balances: balances::Pallet<Self>,
}

impl system::Config for Runtime {
	type AccountID = String;
	type BlockNumber = types::BlockNumber;
	type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
	type Balance = types::Balance;
}

impl Dispatch for Runtime {
	type Caller = <Runtime as system::Config>::AccountID;
	type Call = RuntimeCall;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
		match call {
			RuntimeCall::Balances(call) => self.balances.dispatch(caller, call)?,
		}
		Ok(())
	}
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}

	// Execute a block of extrinsics. Increments the block number.
	fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
		self.system.inc_block_number();
		if self.system.block_number() != block.header.block_number {
			return Err("Invalid block");
		}
		for (i, ext) in block.extrinsics.into_iter().enumerate() {
			self.system.inc_nonce(ext.caller.clone());
			let _res = self.dispatch(ext.caller, ext.call).map_err(|err| {
				eprint!(
					"Extrinsic error\n\tBlock Number {}\n\tExtrinsic Number {}\n\tError {:?}",
					block.header.block_number, i, err
				)
			});
		}
		Ok(())
	}
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
				call: RuntimeCall::Balances(balances::Call::Transfer {
					to: bob.clone(),
					amount: 50,
				}),
			},
			types::Extrinsic {
				caller: alice.clone(),
				call: RuntimeCall::Balances(balances::Call::Transfer {
					to: charlie.clone(),
					amount: 20,
				}),
			},
		],
	};

	runtime.execute_block(block_1).expect("Invalid block");

	println!("{:#?}", runtime);
}
