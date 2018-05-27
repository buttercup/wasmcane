#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate ring;

use wasm_bindgen::prelude::*;
use ring::{digest, pbkdf2};
use std::str;

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn pbkdf2(password: &str, salt: &str, iterations: u32, bits: usize) -> Vec<u8> {
    let mut to_store = Vec::new();
    to_store.resize(bits / 8, 0);
    pbkdf2::derive(DIGEST_ALG, iterations, salt.as_bytes(), password.as_bytes(), &mut to_store);
    to_store
}

#[test]
fn it_works() {
    assert_eq!(add(2, 2), 4);
}
