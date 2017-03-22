#[cfg(test)]
mod test {
    /**
    AES in ECB mode

    The Base64-encoded content in this file has been encrypted via
    AES-128 in ECB mode under the key

    "YELLOW SUBMARINE".

    (case-sensitive, without the quotes; exactly 16 characters; I like
    "YELLOW SUBMARINE" because it's exactly 16 bytes long, and now you
    do too).

    Decrypt it. You know the key, after all.

    Easiest way: use OpenSSL::Cipher and give it AES-128-ECB as the cipher.

    */

    extern crate openssl;

    use strings::*;
    use openssl::symm::{Cipher, Crypter, Mode};
    
    #[test]
    fn test_c07() {
        let src = "data/7.txt";
        let ciphertext = match load_from_file(src) {
		        Err(why) => panic!("Failed to read {}: {}", src, why),
		        Ok(data) => data,
	      };

        let key = "YELLOW SUBMARINE";

        let c = Cipher::aes_128_ecb();
        let mut decrypter = Crypter::new(c, Mode::Decrypt, &key.as_bytes(), None).unwrap();

        let mut decrypted = vec![0; ciphertext.len() + c.block_size()];
        let count = decrypter.update(&ciphertext, &mut decrypted).unwrap();

        //println!("{:?}", count);
        
        let rest = decrypter.finalize(&mut decrypted[count..]).unwrap();

        decrypted.truncate(count + rest);
        
        let string = String::from_utf8(decrypted).unwrap();
        
        //println!("{:?}", string);

        assert!(string.contains("I'm back and I'm ringin' the bell"));
        assert_eq!(string.len(), 2876);
    }
}
