extern crate ring;

use ring::{digest, pbkdf2};
use std::str;

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;

fn pbkdf2(password: &str, salt: &str, iterations: u32, bits: usize) -> Vec<u8> {
    let mut to_store = Vec::new();
    to_store.resize(bits / 8, 0);
    pbkdf2::derive(DIGEST_ALG, iterations, salt.as_bytes(), password.as_bytes(), &mut to_store);
    to_store
}

fn main() {
    let buf = pbkdf2("x!zooj$khaz", "sallarkaboli", 3000, 512);
    println!("result: {:?}", buf);
}
