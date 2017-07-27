#[cfg(test)]
mod test {

    // Take your oracle function from #12. Now generate a random count
    // of random bytes and prepend this string to every plaintext. You
    // are now doing:
    //
    // AES-128-ECB(random-prefix || attacker-controlled || target-bytes, random-key)
    //
    // Same goal: decrypt the target-bytes.
    //
    // Stop and think for a second.
    //
    // What's harder than challenge #12 about doing this? How would
    // you overcome that obstacle? The hint is: you're using all the
    // tools you already have; no crazy math is required.
    //     Think "STIMULUS" and "RESPONSE".
    
    use rustc_serialize::base64::FromBase64;
    use crypto::*;
    use oracle::*;
    use strings::*;
    use std::iter;

    struct Oracle {
        key: String,
        prefix: String,
        ciphertext: Vec<u8>
    }

    impl Oracle {
        fn new(t:Vec<u8>) -> Oracle {
            Oracle {
                // we aren't supposed to know the value of this key!
                key: random_key(16),
                prefix: random_string(),
                ciphertext: t
            }
        }

        // AES-128-ECB(random-prefix || attacker-controlled || target-bytes, random-key)
        fn respond(&self, plaintext:&[u8]) -> Vec<u8> {
            let mut work:Vec<u8> = self.prefix.as_str().as_bytes().to_vec();
            work.extend_from_slice(plaintext);
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

    //
    // 2) Detect that the function is using ECB. You already know, but
    // do this step anyways.
    fn detect_prefix_len(oracle: &Oracle, block_len:usize) -> usize {
        let mut s: String = iter::repeat("x").take(block_len*2).collect();
        let mut count = 0;
        let mut x = oracle.respond(s.as_bytes());
        
        while repeat_count(&x, block_len) < 2 {
            count += 1;
            s.push('x');
            x = oracle.respond(s.as_bytes());
        }

        count % block_len       
    }


    // figure out the number of prefix blocks. this code is
    // borrowed from:
    //
    // https://github.com/Munksgaard/cryptopals/blob/master/chal14/src/main.rs#L124
    //
    // because it's the coolest solution for finding the number of prefix blocks i've seen
    fn find_prefix_blocks(oracle: &Oracle, blocksize: usize) -> usize {
        let tmp = oracle.respond(b"");
        let orig_blocks: Vec<&[u8]> = tmp.chunks(blocksize).collect();
    
        let tmp2 = oracle.respond(b"A");
        let new_blocks: Vec<&[u8]> = tmp2.chunks(blocksize).collect();

        for (i, block) in new_blocks.iter().enumerate() {
            if block != &orig_blocks[i] {
                return i + 1;       // Add one because of the padding
            }
        }

        panic!("Prefix not found");
    }
    
    #[test]
    fn test_c14() {
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

        let prefix_length = detect_prefix_len(&oracle, block_size);
        //println!("PREFIX LEN {:?}", prefix_length);
             
        //
        // 3) Knowing the block size, craft an input block that is exactly
        // 1 byte short (for instance, if the block size is 8 bytes, make
        // "AAAAAAA"). Think about what the oracle function is going to
        // put in that last byte position.

        let cipher_length:usize = oracle.respond(b"").len();
        //println!("The cipher_length is {}", cipher_length);
        
        let number_of_blocks = cipher_length / block_size;
        //println!("Number of blocks: {}", number_of_blocks);

        let prefix_blocks = find_prefix_blocks(&oracle, block_size);
        //println!("prefix blocks: {}", prefix_blocks);

        for block in 0..number_of_blocks-prefix_blocks {
            // For each byte in the block
            for byte in 1..(block_size+1) {
                let mut input = generate_buffer(prefix_length);
                let pad = generate_buffer(block_size - byte);
                input.extend_from_slice(&pad);
                
                let tmp = oracle.respond(&input);
                let target_block = tmp.chunks(block_size).nth(prefix_blocks + block).unwrap();
                
                let from = (block * 16) as usize;
                
                // create a dictionary of every possible result for this byte
                let mut flag = false;
                
                for x in 0..256 {
                    let mut input = generate_buffer(prefix_length);
                    let block_padding = generate_buffer(block_size - byte);
                    input.extend_from_slice(&block_padding);
                    input.extend_from_slice(&result);
                    
                    // push the test character onto the existing results
                    input.push(x as u8);
                    let offset = prefix_blocks * block_size;
                    
                    // run it through the oracle
                    //println!("BBB");
                    let guess_block = oracle.respond(&input);
                    
                    if guess_block[offset + from..offset + from + 16].to_vec() == target_block {
                        result.push(x as u8);
                        flag = true;
                        break;
                    }      
                }
                
                if !flag {
                    break;
                }
                
            }

        } // for prefix_block_count

        
        let output = String::from_utf8(result).unwrap();
        //println!("\n**************************\n{:?}", output);

        let expected = "Rollin' in my 5.0";
        assert!(output.contains(expected));
    }
}
