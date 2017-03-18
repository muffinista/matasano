use rustc_serialize::base64::FromBase64;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::str;
use std::io;


// https://github.com/fotcorn/rust_crypto_challenges/blob/master/src/bin/set1_challenge6.rs#L105
pub fn string_to_blocks(data: &[u8], key_size: usize) -> Vec<Vec<u8>> {
	  let mut blocks: Vec<Vec<u8>> = vec![Vec::new(); key_size];
	  for i in 0..data.len() {
		    blocks[i % key_size].push(data[i]);
	  }
	  return blocks;
}

pub fn load_from_file(src: &str) -> io::Result<Vec<u8>> {
	  let file = try!(File::open(src));
	  let reader = BufReader::new(file);
        
	  let mut data = String::new();
        
	  for line in reader.lines() {
		    data.push_str(&(line.unwrap()));
	  }
    
	  let decoded_data = data.from_base64().unwrap();
	  return Ok(decoded_data);
}
