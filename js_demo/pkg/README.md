# Groth-16 Verifier
### A groth-16 verifier designed to work with WASM in `#[no_std]` environments

This is an imlpementation of the groth 2016 proof verification algorithm. The implementation of the verifier is built upon verifier found in the ark-groth-16 crate from [arkworks-rs](https://arkworks.rs).

## Why this repo?

The ark-groth16 crate is an awesome and optimized implementation but:
* the `#[no_std]` feature requires an allocator when targeting a WebAssembly build, which some personalized WASM VMs don't have since they won't allow to allocate additional resources. The allocator is used in the `hashbrown` crate that is a dependency in the `ark-poly` crate, which is only needed for the circuit and the proof. Since this crate is intended to only be a verifier, I cut off the `ark-poly` dependency completely.
* it's common practice in smart contracts that work with zk-SNARKs to only implement the verification part, that is why this crate focuses only on this task.
* the parameters in the `verify_proof()` function make it fairly intricate to provide already-generated proofs and verifying keys to the pairing engine. For example, passing Zokrates-built proofs and verifying keys would require quite some debugging of many of the ark dependencies in the ark-groth16 crate. This iplementation allows you to pass proofs and verifying keys to the verification function as bytes, which results in the developers not having to make the extra effort of debugging the crate to verify a zk-SNARK.
* This repo offers some helpers to help integrating this verifier on stellar native contracts: Soroban.
* this repo provides some docs.

## This repo is not for
* those who have no need verifying "external" proofs.
* those who need to build arithmetic circuits, proofs and keys with the crate.
* those that aren't targeting a WASM-unwknown-unknwon `#[no_std]` build.

If you aren't searching for those things, I recommend checking out the awesome [groth16](https://github.com/arkworks-rs/groth16) repo.

## This repo is for
* those who want to verify zk-SNARKs from a `#[no_std]` WASM environment that doesn't support allocation, like Soroban smart contracts.

## License

This library is licensed under either of the following licenses, at your discretion.

 * Apache License Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

Unless you explicitly state otherwise, any contribution submitted for inclusion in this library by you shall be dual licensed as above (as defined in the Apache v2 License), without any additional terms or conditions.

## Acknowledgements

This verifier is completely reliant on the [arkworks](https://arkworks.rs/) ecosystem and their amazing work.
