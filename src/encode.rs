extern crate rustc_serialize;
use rustc_serialize::hex::{ToHex};

pub fn xor_encode(src: &str, key: &str) -> String {
    let key = String::from(key).into_bytes();
    let mut out = String::from(src).into_bytes();
    
    let mut iter = key.iter().cycle();
    for i in 0..out.len() {
        out[i] ^= *iter.next().unwrap();
    }

    out.to_hex()
}
    
