#[cfg(test)]
mod test {
    
    /**

    There's a file here. It's been base64'd after being encrypted with repeating-key XOR.

    Decrypt it.

    Here's how:

    Let KEYSIZE be the guessed length of the key; try values from 2 to (say) 40.

    For each KEYSIZE, take the first KEYSIZE worth of bytes, and the
    second KEYSIZE worth of bytes, and find the edit distance between
    them. Normalize this result by dividing by KEYSIZE.


    The KEYSIZE with the smallest normalized edit distance is probably
    the key. You could proceed perhaps with the smallest 2-3 KEYSIZE
    values. Or take 4 KEYSIZE blocks instead of 2 and average the
    distances.


    Now that you probably know the KEYSIZE: break the ciphertext into
    blocks of KEYSIZE length.

    Now transpose the blocks: make a block that is the first byte of
    every block, and a block that is the second byte of every block,
    and so on.

    Solve each block as if it was single-character XOR. You already
    have code to do this.

    For each block, the single-byte XOR key that produces the best
    looking histogram is the repeating-key XOR key byte for that
    block. Put them together and you have the key.

    This code is going to turn out to be surprisingly useful later on.
    Breaking repeating-key XOR ("Vigenere") statistically is obviously
    an academic exercise, a "Crypto 101" thing. But more people "know
    how" to break it than can actually break it, and a similar
    technique breaks something much more important.
    */

    
    use scores::*;
    use decode::guess_xor;

    use strings::*;
    
    
    #[test]
    fn test_c06() {
        let buffer = match load_from_file("data/6.txt") {
		        Err(why) => panic!("Failed to read data/set1_challenge4.txt: {}", why),
		        Ok(data) => data,
	      };

        let scores  = hamming_dist_for_keysizes(&buffer, 2, 40);       

        let d = &scores[0];

        //println!("{} {:?}", d.length, d.dist);

        let blocks = string_to_blocks(&buffer, d.length);
        
        let mut key: Vec<u8> = vec!(0; d.length);
	      for i in 0..blocks.len() {
		        key[i] = guess_xor(&blocks[i]);
	      }
        
        let mut result: Vec<u8> = Vec::new();
        for i in 0..buffer.len() {
            result.push(buffer[i] ^ key[i % d.length]);
        }
        
        let string = String::from_utf8(result).unwrap();

        assert!(string.contains("I'm back and I'm ringin' the bell"));
    }
}
