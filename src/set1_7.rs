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

    use strings::*;
    use crypto::*;
    
    #[test]
    fn test_c07() {
        let src = "data/7.txt";
        let ciphertext = match load_base64_from_file(src) {
		        Err(why) => panic!("Failed to read {}: {}", src, why),
		        Ok(data) => data,
	      };

        let key = "YELLOW SUBMARINE";
        let string = String::from_utf8(decrypt_ecb_chunk(ciphertext, key)).unwrap();
        
        assert!(string.contains("I'm back and I'm ringin' the bell"));
        assert_eq!(string.len(), 2876);
    }
}
