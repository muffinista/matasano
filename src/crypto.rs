extern crate openssl;
use openssl::symm::{Cipher, Crypter, Mode};

pub fn decrypt_ecb_chunk(ciphertext:Vec<u8>, key:&str) -> Vec<u8> {
    let c = Cipher::aes_128_ecb();
    let mut decrypter = Crypter::new(c, Mode::Decrypt, &key.as_bytes(), None).unwrap();

    let mut decrypted = vec![0; ciphertext.len() + c.block_size()];
    let count = decrypter.update(&ciphertext, &mut decrypted).unwrap();

    let rest = decrypter.finalize(&mut decrypted[count..]).unwrap();

    decrypted.truncate(count + rest);
    decrypted
}

pub fn encrypt_ecb_chunk(ciphertext:Vec<u8>, key:&str) -> Vec<u8> {
    let c = Cipher::aes_128_ecb();
    let mut encrypter = Crypter::new(c, Mode::Encrypt, &key.as_bytes(), None).unwrap();

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
