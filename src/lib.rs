#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate rcw;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use rcw::pbkdf2::pbkdf2;
use rcw::hmac::Hmac;
use rcw::sha2::Sha256;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn pbkdf2_derive(password: &str, salt: &str, iterations: u32, bits: usize) -> Vec<u8> {
    let mut to_store = Vec::new();
    let mut mac = Hmac::new(Sha256::new(), password.as_bytes());

    to_store.resize(bits / 8, 0);
    pbkdf2(&mut mac, salt.as_bytes(), iterations, &mut to_store);
    to_store
}

#[test]
fn it_works() {
    let buf = pbkdf2_derive("password", "salt", 500, 512);
    assert_eq!(buf.len(), 64);
    assert!(buf[0] > 0);
    assert!(buf[63] > 0);
}
