extern crate rustc_serialize;
use rustc_serialize::hex::{FromHex};
use scores::string_score;

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

pub fn xor_decode2(x: &[u8], c: u8) -> String {

    let mut out:Vec<u8> = vec![];

    for i in 0..x.len() {
        out.push(x[i] ^ c);
    }

    let result = String::from_utf8(out);

    if ! result.is_err() {
        result.unwrap()
    }
    else {
        String::new()
    }
}

pub fn guess_xor(x: &[u8]) -> u8 {
    let mut best: u8 = 0;
    let mut max = 0;

    for char in (0..255).rev() {
        let foo = xor_decode2(&x, char);
        let val = string_score(&foo);

        //println!("{:?} {:?} {:?}", foo, char, val);

        
        if val > max {
            best = char;
            max = val;
        }
    }

    return best;    
}
