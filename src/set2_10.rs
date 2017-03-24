#[cfg(test)]
mod test {
    // Implement CBC mode

    // CBC mode is a block cipher mode that allows us to encrypt
    // irregularly-sized messages, despite the fact that a block cipher
    // natively only transforms individual blocks.

    // In CBC mode, each ciphertext block is added to the next plaintext
    // block before the next call to the cipher core.

    // The first plaintext block, which has no associated previous ciphertext
    // block, is added to a "fake 0th ciphertext block" called the
    // initialization vector, or IV.

    // Implement CBC mode by hand by taking the ECB function you wrote
    // earlier, making it encrypt instead of decrypt (verify this by
    // decrypting whatever you encrypt to test), and using your XOR function
    // from the previous exercise to combine them.

    // The file here is intelligible (somewhat) when CBC decrypted against
    // "YELLOW SUBMARINE" with an IV of all ASCII 0 (\x00\x00\x00 &c)

    use strings::*;
    use crypto::*;
    
    #[test]
    fn test_c10() {
        let key = "YELLOW SUBMARINE";

        let src = "data/10.txt";
        let tmp = match load_from_file(src) {
		        Err(why) => panic!("Failed to read {}: {}", src, why),
		        Ok(data) => data,
	      };

        let output = decrypt_cbc(&tmp, key, key.len());

        let string = String::from_utf8_lossy(&output);
        assert!(string.contains("I'm back and I'm ringin' the bell"));
    }
}
