# Groth-16 Verifier
### A groth-16 verifier designed to work with WASM in `#[no_std]` environments

This is an imlpementation of the groth 2016 proof verification algorithm. This crate is reliant on the awesome `ark-std`, `ark-bls12_377`, `ark-ec` and `ark-ff` crates from [arkworks-rs](https://arkworks.rs).

As you can see in the demo, you can use this crate to verify zk-SNARKs on your browser by compiling the crate to WebAssembly, you can look at the `js_demo` directory in the code to implement it.

You can also try out the verifier from your browser [here](https://heytdep.github.io/wasm-groth16-verifier/).

By default this library makes no use of the `std` crate, however it uses the `alloc` crate.

## License

This library is licensed under either of the following licenses, at your discretion.

 * Apache License Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

Unless you explicitly state otherwise, any contribution submitted for inclusion in this library by you shall be dual licensed as above (as defined in the Apache v2 License), without any additional terms or conditions.

## Acknowledgements

This verifier is completely reliant on the [arkworks](https://arkworks.rs/) ecosystem and their amazing work.
