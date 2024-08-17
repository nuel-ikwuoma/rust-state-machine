# Rust state machine
This repository hosts the solution to the polkadot SDK style implementation of a rust state machine following this excellent
[tutorial](https://github.com/shawntabrizi/rust-state-machine/tree/gitorial/)  by [Shawn](https://x.com/shawntabrizi?lang=en). 


## How the repository is structured

### Pallets implementation
> Pallets are the building blocks, put together to compose the blockchain runtime. The set of functions which are exposed to agents external to the blockchain aka Extrinsics (for simplicity) are defined therein.
 - [*balances pallet*](./src/balances.rs)
 - [*systems pallet*](./src/system.rs)
 - [*proof of existence pallet*](./src/poe.rs) 
 
 ### Support logic
 > This module defines generic representations of some basic primitives such as *Block*, *Block Header* etc and trait implementation to be re-used across the pallets.
 
 - [*support*](./src/support.rs) 

### Runtime
> The runtime serves as the entry point into the system and concretely defines an implementation for the generic entities of the pallets.

 - [runtime](./src/main.rs#Runtime) 