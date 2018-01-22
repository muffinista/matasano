extern crate openssl;
use openssl::symm::{Cipher, Crypter, Mode};

use encode::*;

pub fn decrypt_ecb_chunk(ciphertext:Vec<u8>, key:&str) -> Vec<u8> {
    let c = Cipher::aes_128_ecb();
    
    let mut decrypter = Crypter::new(c, Mode::Decrypt, &key.as_bytes(), None).unwrap();

    // set padding to false if the length of the incoming data equals
    // the CBC block size
    decrypter.pad(ciphertext.len() != c.block_size());

    let mut decrypted = vec![0; ciphertext.len() + c.block_size()];
    let count = decrypter.update(&ciphertext, &mut decrypted).unwrap();
    let rest = decrypter.finalize(&mut decrypted[count..]).unwrap();
    
    decrypted.truncate(count + rest);
    decrypted
}

pub fn encrypt_ecb_chunk(ciphertext:Vec<u8>, key:&str) -> Vec<u8> {
    let c = Cipher::aes_128_ecb();
    let mut encrypter = Crypter::new(c, Mode::Encrypt, &key.as_bytes(), None).unwrap();
    encrypter.pad(ciphertext.len() != c.block_size());

    // set padding to false if the length of the incoming data equals
    // the CBC block size
    let mut encrypted = vec![0; ciphertext.len() + c.block_size()];

    let count = encrypter.update(&ciphertext, &mut encrypted).unwrap();

    let rest = encrypter.finalize(&mut encrypted[count..]).unwrap();

    encrypted.truncate(count + rest);

    encrypted
}


#[test]
fn test_ecb_encrypt_and_decrypt() {
    let s = "Good morning. In less than an hour, aircraft from here will join others from around the world. And you will be launching the largest aerial battle in this history of mankind.";
    
    let key = "YELLOW SUBMARINE";
    
    let e = encrypt_ecb_chunk(s.as_bytes().to_vec(), key);

    let d = String::from_utf8(decrypt_ecb_chunk(e, key)).unwrap();

    assert_eq!(s, d);
}


pub fn decrypt_cbc(ciphertext:&[u8], key:&str, size:usize) -> Vec<u8> {
    let mut block2:Vec<u8> = vec![0; size];
    let mut output:Vec<u8> = Vec::new();

    let blocks:Vec<_> = ciphertext.chunks(size).collect();

	  for i in 0..blocks.len() {
        let decoded = decrypt_ecb_chunk(blocks[i].to_vec(), key);
        let decoded_xor = xor_encode(&decoded, &block2);

        output.extend_from_slice(&decoded_xor);
        
        block2 = blocks[i].to_vec();
    }
    
    output
}


pub fn encrypt_cbc(plaintext:&[u8], key:&str, size:usize) -> Vec<u8> {
    let mut block2:Vec<u8> = vec![0; size];
    let mut output:Vec<u8> = Vec::new();

    let blocks:Vec<_> = plaintext.chunks(size).collect();

	  for i in 0..blocks.len() {
        let encoded_xor = xor_encode(&blocks[i].to_vec(), &block2);
        let encoded = encrypt_ecb_chunk(encoded_xor, key);

        output.extend_from_slice(&encoded);
        
        block2 = encoded;
    }
    
    output
}

/*
#[test]
fn test_decrypt_cbc() {
    let s = "Good morning. In less than an hour, aircraft from here will join others from around the world. And you will be launching the largest aerial battle in this history of mankind.";
    let key = "YELLOW SUBMARINE";

    let e = encrypt_cbc(&s.as_bytes().to_vec(), key, 16);
    let d = decrypt_cbc(&e, key, 16);

    let result = String::from_utf8_lossy(&d);

    assert!(result.contains(s));
}*/