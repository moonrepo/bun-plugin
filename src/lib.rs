// WASM cannot be executed through the test runner and we need to avoid building
// WASM code for non-WASM targets. We can solve both of these with a cfg flag.

#[cfg(not(test))]
mod proto;

#[cfg(not(test))]
pub use proto::*;
