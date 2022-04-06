#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
//! Crate that provides the bindings to interact with the Desmos blockchain custom modules from a CosmWasm
//! smart contract.

#[cfg(all(not(target_arch = "wasm32"), feature = "mocks"))]
pub mod mock;

#[cfg(feature = "profiles")]
pub mod profiles;

#[cfg(feature = "relationships")]
pub mod relationships;

#[cfg(feature = "subspaces")]
pub mod subspaces;

#[cfg(feature = "msg")]
pub mod msg;

#[cfg(feature = "query")]
pub mod query;

pub mod iter;
pub mod types;
