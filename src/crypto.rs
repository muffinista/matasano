extern crate openssl;
use openssl::symm::{Cipher, Crypter, Mode};

pub fn decrypt_ecb_chunk(ciphertext:Vec<u8>, key:&str) -> String {
    let c = Cipher::aes_128_ecb();
    let mut decrypter = Crypter::new(c, Mode::Decrypt, &key.as_bytes(), None).unwrap();

    let mut decrypted = vec![0; ciphertext.len() + c.block_size()];
    let count = decrypter.update(&ciphertext, &mut decrypted).unwrap();

    //println!("{:?}", count);
        
    let rest = decrypter.finalize(&mut decrypted[count..]).unwrap();

    decrypted.truncate(count + rest);
        
    String::from_utf8(decrypted).unwrap()
}
