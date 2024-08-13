mod balances;
mod system;

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet,
	balances: balances::Pallet,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		/* TODO: Create a new `Runtime` by creating new instances of `system` and `balances`. */
		Self {
			system: system::Pallet::new(),
			balances: balances::Pallet::new(),
		}
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

	// increment the block number
	runtime.system.inc_block_number();
	assert_eq!(runtime.system.block_number(), 1);

	// first transaction
	runtime.system.inc_nonce(alice.clone());
	let _res = runtime.balances.transfer(alice.clone(), bob.clone(), 50).map_err(|err| eprint!("{:?}", err));

	runtime.system.inc_nonce(alice.clone());
	let _res = runtime.balances.transfer(alice.clone(), charlie.clone(), 20);

	println!("{:#?}", runtime);
}
