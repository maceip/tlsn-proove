//! Implements a hello-world example for Arbitrum Stylus, providing a Solidity ABI-equivalent
//! Rust implementation of the Counter contract example provided by Foundry.
//! Warning: this code is a template only and has not been audited.
//! ```
//! contract Counter {
//!     uint256 public number;
//!     function setNumber(uint256 newNumber) public {
//!         number = newNumber;
//!     }
//!     function increment() public {
//!         number++;
//!     }
//! }
//! ```

// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

/// Initializes a custom, global allocator for Rust programs compiled to WASM.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Import the Stylus SDK along with alloy primitive types for use in our program.
use stylus_sdk::{alloy_primitives::U256, prelude::*};

mod tlsnprover;
const SIG_LEN: usize = 96;
const PUBKEY_LEN: usize = 48;
// Define the entrypoint as a Solidity storage object, in this case a struct
// called `Counter` with a single uint256 value called `number`. The sol_storage! macro
// will generate Rust-equivalent structs with all fields mapped to Solidity-equivalent
// storage slots and types.
#[solidity_storage]
#[entrypoint]
pub struct TLSNProver;

#[external]
impl TLSNProver {
    pub fn prove(&self, data: Bytes) -> Result<(), Vec<u8>> {
        if data.len() <= PUBKEY_LEN + SIG_LEN {
            return Err("data does not include signed message".as_bytes().to_vec());
        }
        let mut data = data.as_slice();
        let sig = &data[..SIG_LEN];
        data = &data[SIG_LEN..];
        let pubkey = &data[..PUBKEY_LEN];
        data = &data[PUBKEY_LEN..];
        let msg = &data;
        match tlsnprover::prove(sig, msg, pubkey) {
            Ok(()) => Ok(()),
            Err(()) => Err("sig failed to verify".as_bytes().to_vec()),
        }
    }
}