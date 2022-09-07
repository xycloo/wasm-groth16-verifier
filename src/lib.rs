//! An implementation of the [`Groth16`] zkSNARK.
//!
//! [`Groth16`]: https://eprint.iacr.org/2016/260.pdf
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    missing_docs
)]
//#![feature(slice_pattern)]
#![allow(clippy::many_single_char_names, clippy::op_ref)]
use wasm_bindgen::prelude::*;

/// Data structures used by the verifier.
pub mod data_structures;
/// Building a proof from strings
pub mod proof_wrap;
/// Utils to verify the proof
pub mod verify_utils;
/// Building a verifying key from strings
pub mod verifying_key_wrap;

use core::str::FromStr;

use ark_bls12_377::Bls12_377;
use js_sys::Array;

use crate::verifying_key_wrap::build_vk;

pub use self::data_structures::*;

use ark_std::string::String;
use ark_std::vec::Vec;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/*
enable this macro for logging:

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
*/

/// verify proofs from strings of decimals
#[wasm_bindgen]
pub fn verify_external(
    vk_a: Array,
    vk_b: Array,
    vk_g: Array,
    vk_d: Array,
    vk_g_abc: Array,
    p_a: Array,
    p_b: Array,
    p_c: Array,
    pub_f: Array,
) -> bool {
    use crate::proof_wrap::build_proof;
    use crate::verify_utils::{prepare_vk, verify};
    use ark_bls12_377::Fr;
    use ark_ff::Fp256;
    let mut abc_vec: Vec<[String; 2]> = Vec::new();
    let mut iters = 0;
    let mut i = 1;
    for _ in 0..vk_g_abc.length() {
        if i != 2 {
            i += 1;
        } else {
            if iters > 0 {
                iters += 1;
                abc_vec.push([
                    vk_g_abc.get(iters + (iters - 2)).as_string().unwrap(),
                    vk_g_abc.get(iters + (iters - 2) + 1).as_string().unwrap(),
                ]);
                i = 1;
            } else {
                iters += 1;
                abc_vec.push([
                    vk_g_abc.get(iters * (i - 2)).as_string().unwrap(),
                    vk_g_abc.get(iters * (i - 1)).as_string().unwrap(),
                ]);
                i = 1;
            }
        }
    }

    let mut va_vec: Vec<String> = Vec::new();
    for el in vk_a.to_vec() {
        va_vec.push(el.as_string().unwrap())
    }
    let va: Vec<&str> = va_vec.iter().map(|s| &**s).collect();
    let vk = build_vk::<Bls12_377>(
        va.as_slice(),
        (
            [
                &vk_b.get(0).as_string().unwrap(),
                &vk_b.get(1).as_string().unwrap(),
            ],
            [
                &vk_b.get(2).as_string().unwrap(),
                &vk_b.get(3).as_string().unwrap(),
            ],
        ),
        (
            [
                &vk_g.get(0).as_string().unwrap(),
                &vk_g.get(1).as_string().unwrap(),
            ],
            [
                &vk_g.get(2).as_string().unwrap(),
                &vk_g.get(3).as_string().unwrap(),
            ],
        ),
        (
            [
                &vk_d.get(0).as_string().unwrap(),
                &vk_d.get(1).as_string().unwrap(),
            ],
            [
                &vk_d.get(2).as_string().unwrap(),
                &vk_d.get(3).as_string().unwrap(),
            ],
        ),
        abc_vec.as_slice(),
    );

    let prep_vk = prepare_vk(&vk);
    let mut pa_vec: Vec<String> = Vec::new();
    for el in p_a.to_vec() {
        pa_vec.push(el.as_string().unwrap())
    }
    let pa: Vec<&str> = pa_vec.iter().map(|s| &**s).collect();

    let mut pc_vec: Vec<String> = Vec::new();
    for el in p_c.to_vec() {
        pc_vec.push(el.as_string().unwrap())
    }

    let pc: Vec<&str> = pc_vec.iter().map(|s| &**s).collect();

    let proof = build_proof::<Bls12_377>(
        pa.as_slice(),
        (
            [
                &p_b.get(0).as_string().unwrap(),
                &p_b.get(1).as_string().unwrap(),
            ],
            [
                &p_b.get(2).as_string().unwrap(),
                &p_b.get(3).as_string().unwrap(),
            ],
        ),
        pc.as_slice(),
    );

    let mut pub_inputs_vec: Vec<Fr> = Vec::new();
    for img in pub_f.to_vec() {
        pub_inputs_vec.push(Fp256::from_str(&img.as_string().unwrap()).unwrap())
    }

    verify(proof, prep_vk, pub_inputs_vec.as_slice())
}
