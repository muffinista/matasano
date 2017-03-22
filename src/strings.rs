use rustc_serialize::base64::FromBase64;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::str;
use std::io;



//
// take a vector of bytes in the form of AAAABBBBCCCCDDDD
// and split it into a vector of byte vectors in the format:
// [
//   [ABCD]
//   [ABCD]
//   [ABCD]
//   [ABCD]
// ]
//
// @see https://github.com/fotcorn/rust_crypto_challenges/blob/master/src/bin/set1_challenge6.rs#L105
//
//
pub fn transpose_string(data: &[u8], key_size: usize) -> Vec<Vec<u8>> {
	  let mut blocks: Vec<Vec<u8>> = vec![Vec::new(); key_size];
	  for i in 0..data.len() {
		    blocks[i % key_size].push(data[i]);
	  }
	  return blocks;
}


#[test]
fn test_transpose_string() {
    let src = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let result = transpose_string(&src, 4);
    assert_eq!(0, result[0][0]);
    assert_eq!(4, result[0][1]);
    assert_eq!(8, result[0][2]);
    assert_eq!(12, result[0][3]);
}


///
/// compare to vectors of bytes and see if they are equal
/// @see http://stackoverflow.com/questions/40767815/how-do-i-check-whether-a-vector-is-equal-to-another-vector-that-contains-nan-and
pub fn byte_vec_compare(va: &[u8], vb: &[u8]) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
        va.iter()
        .zip(vb)
        .all(|(a,b)| *a == *b)
}

#[test]
fn test_byte_vec_compare() {
    assert!(byte_vec_compare(&[0, 1, 2, 3], &[0, 1, 2, 3]));
    assert!(!byte_vec_compare(&[0, 2, 1, 3], &[0, 1, 2, 3]));
    assert!(!byte_vec_compare(&[0, 1, 2, 3, 4], &[0, 1, 2, 3]));
}


///
/// load a text file into a vector of bytes
///
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

///
/// given a byte vector, split it into chunks of size 'size' and see if any of them
/// repeat. this is basically a check for use of ECB
///
pub fn repeat_count(s: &[u8], size:usize) -> usize {
    let mut max_count = 1;
    let blocks:Vec<_> = s.chunks(size).collect();
	  for i in 0..blocks.len() {
        let b = blocks[i];
        
        let count = blocks.iter().map(|x| byte_vec_compare(x, b) ).filter(|x| *x).collect::<Vec<bool>>().len();
        if count > max_count {
            max_count = count;
        }                
	  }

    return max_count;
}


#[test]
fn test_repeat_count() {
    let data = [
        0, 0, 0, 1,
        2, 3, 0, 1,
        2, 4, 0, 0,
        0, 0, 0, 1,
        2, 4, 0, 1];
    assert_eq!(2, repeat_count(&data, 4));
}
