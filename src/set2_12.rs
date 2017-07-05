#[cfg(test)]
mod test {

    //
    // Byte-at-a-time ECB decryption (Simple)
    //

    //
    // Copy your oracle function to a new function that encrypts
    // buffers under ECB mode using a consistent but unknown key (for
    // instance, assign a single random key, once, to a global
    // variable).
    //
    //
    // Now take that same function and have it append to the
    // plaintext, BEFORE ENCRYPTING, the following string:
    //
    // Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg
    // aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq
    // dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg
    // YnkK
    //
    // Spoiler alert.
    // Do not decode this string now. Don't do it.
    //
    // Base64 decode the string before appending it. Do not base64
    // decode the string by hand; make your code do it. The point is
    // that you don't know its contents.
    //
    // What you have now is a function that produces:
    // AES-128-ECB(your-string || unknown-string, random-key)
    // It turns out: you can decrypt "unknown-string" with repeated
    // calls to the oracle function!
    //
    // Here's roughly how:
    //
    // 1) Feed identical bytes of your-string to the function 1 at a
    // time --- start with 1 byte ("A"), then "AA", then "AAA" and so
    // on. Discover the block size of the cipher. You know it, but do
    // this step anyway.
    //
    // 2) Detect that the function is using ECB. You already know, but
    // do this step anyways.
    //
    // 3) Knowing the block size, craft an input block that is exactly
    // 1 byte short (for instance, if the block size is 8 bytes, make
    // "AAAAAAA"). Think about what the oracle function is going to
    // put in that last byte position.
    //
    // 4) Make a dictionary of every possible last byte by feeding
    // different strings to the oracle; for instance, "AAAAAAAA",
    // "AAAAAAAB", "AAAAAAAC", remembering the first block of each
    // invocation.
    //
    // 5) Match the output of the one-byte-short input to one of the
    // entries in your dictionary. You've now discovered the first
    // byte of unknown-string.
    //
    // 6) Repeat for the next byte.
    //
    //
    // Congratulations.
    //
    // This is the first challenge we've given you whose solution will
    // break real crypto. Lots of people know that when you encrypt
    // something in ECB mode, you can see penguins through it. Not so
    // many of them can decrypt the contents of those ciphertexts, and
    // now you can. If our experience is any guideline, this attack
    // will get you code execution in security tests about once a
    // year.

    use rustc_serialize::base64::FromBase64;
    use crypto::*;
    use oracle::*;
    use strings::*;
    use std::iter;

    struct Oracle {
        key: String,
        ciphertext: Vec<u8>
    }

    impl Oracle {
        fn new(t:Vec<u8>) -> Oracle {
            Oracle {
                // we aren't supposed to know the value of this key!
                key: random_key(16),
                ciphertext: t
            }
        }
        fn respond(&self, plaintext:&[u8]) -> Vec<u8> {
            let mut work:Vec<u8> = plaintext.to_vec();

            // Now take that same function and have it append to the plaintext,
            // BEFORE ENCRYPTING, the following string
            work.extend_from_slice(self.ciphertext.as_slice());
            
            let result:Vec<u8> = encrypt_ecb_chunk(work, &self.key);
            result
        }
    }

    fn get_block_size(oracle: &Oracle, data:&[u8]) -> usize {
        oracle.respond(data).len()
    }
    
    // 1) Feed identical bytes of your-string to the function 1 at a
    // time --- start with 1 byte ("A"), then "AA", then "AAA" and so
    // on. Discover the block size of the cipher. You know it, but do
    // this step anyway.
    fn guess_block_size(oracle: &Oracle) -> usize {
        let test_char = 7u8;
        let mut work:Vec<u8> = vec![test_char];
        let base_len = get_block_size(oracle, &work);
        let mut bump_len = base_len;
        
        while bump_len == base_len {
            work.push(test_char);
            bump_len = get_block_size(oracle, &work);
        }
        
        bump_len - base_len
    }

    //
    // 2) Detect that the function is using ECB. You already know, but
    // do this step anyways.
    fn detect_ecb(oracle: &Oracle, block_len:usize) -> bool {
        let s: String = iter::repeat("x").take(256).collect();

        let x = oracle.respond(s.as_bytes());
        let count = repeat_count(&x, block_len);
        count > 1
    }
    
    
    #[test]
    fn test_c12() {       
        let src = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkg\
                   aGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBq\
                   dXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUg\
                   YnkK";

        let mut result: Vec<u8> = Vec::new();
        let ciphertext = src.from_base64().unwrap();
        let oracle = Oracle::new(ciphertext);

        let block_size = guess_block_size(&oracle);
        //println!("------------------------------ {:?}", block_size);
        assert_eq!(block_size, 16);
        
        let using_ecb = detect_ecb(&oracle, block_size);
        assert!(using_ecb);
        
        //
        // 3) Knowing the block size, craft an input block that is exactly
        // 1 byte short (for instance, if the block size is 8 bytes, make
        // "AAAAAAA"). Think about what the oracle function is going to
        // put in that last byte position.

        let cipher_length:usize = oracle.respond(b"").len();
        //println!("The cipher_length is {}", cipher_length);
        
        let number_of_blocks = cipher_length / block_size;
        //println!("Number of blocks: {}", number_of_blocks);


        for block in 0..number_of_blocks {
            let bytes_in_block = if block < number_of_blocks-1 {
                block_size
            } else {
                block_size - (cipher_length as usize % block_size)
            };
            
            // For each byte in the block
            for byte in 1..(bytes_in_block+1) {
                let pad: Vec<u8> = (0..(block_size - byte) as usize)
                    .map(|_| 'A' as u8)
                    .collect();

                let tmp = oracle.respond(&pad);
                
                let from = (block * 16) as usize;
                let to = (block * 16 + (byte - 1)) as usize;

                // create a dictionary of every possible result for this byte
                let mut dict = Vec::new();
                for x in 0..256 {
                    let mut buf: Vec<u8>;
                    // special handling for first block
                    if block == 0 {
                        buf = (0..(block_size - byte) as usize)
                            .map(|_| 'A' as u8)
                            .collect();
                        buf.extend_from_slice(&result[from..to]);
                    } else {
                        buf = result[(result.len() - (block_size-1))..].to_vec();
                    }

                    // push the test character onto the existing results
                    buf.push(x as u8);

                    // run it through the oracle
                    dict.push(oracle.respond(&buf)[..block_size].to_vec());
                }

                // find a match and add it to our results!
                match dict.iter().position(|ref x| x.as_slice() == &tmp[from..from+block_size]) {
                    Some(x) => result.push(x as u8),
                    None => (),
                }
            }
        }

        let output = String::from_utf8(result).unwrap();
        //println!("\n{}", output);

        let expected = "Rollin' in my 5.0";
        assert!(output.contains(expected));
    }
}
