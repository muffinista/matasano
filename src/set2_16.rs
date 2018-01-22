#[cfg(test)]
mod test {
    // CBC bitflipping attacks
    // Generate a random AES key.
    //
    // Combine your padding code and CBC code to write two functions.
    //
    // The first function should take an arbitrary input string, prepend the string:
    //
    // "comment1=cooking%20MCs;userdata="
    // .. and append the string:
    //
    // ";comment2=%20like%20a%20pound%20of%20bacon"
    // The function should quote out the ";" and "=" characters.
    //
    // The function should then pad out the input to the 16-byte AES
    // block length and encrypt it under the random AES key.
    //
    // The second function should decrypt the string and look for the
    // characters ";admin=true;" (or, equivalently, decrypt, split the
    // string on ";", convert each resulting string into 2-tuples, and
    // look for the "admin" tuple).
    //
    // Return true or false based on whether the string exists.
    //
    // If you've written the first function properly, it should not be
    // possible to provide user input to it that will generate the
    // string the second function is looking for. We'll have to break
    // the crypto to do that.
    //
    // Instead, modify the ciphertext (without knowledge of the AES
    // key) to accomplish this.
    //
    // You're relying on the fact that in CBC mode, a 1-bit error in a ciphertext block:
    // Completely scrambles the block the error occurs in
    // Produces the identical 1-bit error(/edit) in the next ciphertext block.
    //


    use oracle::*;
    use crypto::*;
    use strings::*;
    use percent_encoding::{utf8_percent_encode, SIMPLE_ENCODE_SET};

    // most of the encoding options in percent_encoding are too
    // smart for this test, so we'll define an arbitrary set of
    // characters to handle
    define_encode_set! {
        pub CRYPTOPALS_ENCODE_SET = [SIMPLE_ENCODE_SET] | {';', '=', ' '}
    }
    
    struct Oracle {
        key: String,
        prefix: Vec<u8>,
        postfix: Vec<u8>
    }

    impl Oracle {
        fn new() -> Oracle {
            Oracle {
                // we aren't supposed to know the value of this key!
                key: random_key(16),
                prefix: utf8_percent_encode("cooking MCs", CRYPTOPALS_ENCODE_SET).to_string().into_bytes(),
                postfix: utf8_percent_encode(" like a pound of bacon", CRYPTOPALS_ENCODE_SET).to_string().into_bytes()
            }
        }

        fn respond(&self, plaintext:&str) -> Vec<u8> {
            let mut work:Vec<u8> = Vec::new();
            work.extend_from_slice("comment1=".as_bytes());
            work.extend_from_slice(&self.prefix);

            let encoded = utf8_percent_encode(plaintext, CRYPTOPALS_ENCODE_SET).to_string();

            work.extend_from_slice(";userdata=".as_bytes());
            work.extend_from_slice(encoded.as_bytes());

            work.extend_from_slice(";comment2=".as_bytes());
            work.extend_from_slice(self.postfix.as_slice());

            let result:Vec<u8> = encrypt_cbc(&work.to_vec(), &self.key, 16);
            result
        }

        fn decrypt(&self, work:&[u8]) -> Vec<u8> {
            let result:Vec<u8> = decrypt_cbc(&work.to_vec(), &self.key, 16);
            result
        }

        fn is_admin(&self, data:&[u8]) -> bool {
            let result = self.decrypt(data);
            let string = String::from_utf8_lossy(&result);

            string.find(";admin=true;") != None
        }
    }

    
    #[test]
    fn test_c16() {
        let blocksize = 16;
        let oracle = Oracle::new();
        let padstr = generate_buffer(blocksize);

        let guts = vec![String::from_utf8_lossy(&padstr).into_owned(),
                        chr!(';' as u8 ^ 1).to_string(),
                        "admin".to_string(),
                        chr!('=' as u8 ^ 1).to_string(),
                        "true".to_string()];

        // concat -- https://doc.rust-lang.org/std/slice/trait.SliceConcatExt.html#tymethod.join
        let userdata = guts.concat();


        //           1         2         3     
        // 0123456789012345678901234567890123456789012345678901234567890123456789
        // comment1=cooking%20MCs;userdata=AAAAAAAAAAAAAAAA:admin<true;comment2=%20like%20a%20pound%20of%20bacon

        // we're basically cheating here -- 32 is the index of the first
        // A in our padding block, once everything has been escaped/etc
        let flip_idx1 = 32;
        let flip_idx2 = flip_idx1 + "admin=".len();
       
        let mut tmp = oracle.respond(&userdata);

        // not admin yet!
        assert!(!oracle.is_admin(&tmp));

        tmp[flip_idx1] ^= 1;
        tmp[flip_idx2] ^= 1;

        // now we are an admin!
        assert!(oracle.is_admin(&tmp));
       
    }

}
