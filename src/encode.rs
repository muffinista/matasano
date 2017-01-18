extern crate rustc_serialize;
use rustc_serialize::hex::{ToHex};

pub fn xor_encode(src: &str, key: &str) -> String {
    let key = String::from(key).into_bytes();
    let mut out = String::from(src).into_bytes();
    
    let mut key_index = 0;
    for i in 0..out.len() {
        out[i] ^= key[key_index];
        key_index +=1;
        if key_index >= key.len() {
            key_index = 0;
        }
    }
    //String::from_utf8(out).unwrap().to_hex()
    out.to_hex()
}
    
