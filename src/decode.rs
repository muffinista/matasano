extern crate rustc_serialize;
use rustc_serialize::hex::{FromHex};

pub fn xor_decode(x: &str, c: u8) -> String {
    let mut out = x.from_hex().unwrap();

    for i in 0..out.len() {
        out[i] ^= c;
    }

    let result = String::from_utf8(out);

    if ! result.is_err() {
        result.unwrap()
    }
    else {
        String::new()
    }
}
    
