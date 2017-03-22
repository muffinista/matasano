#[cfg(test)]
mod test {
    /**

    Detect AES in ECB mode
    In this file are a bunch of hex-encoded ciphertexts.

    One of them has been encrypted with ECB.

    Detect it.

    Remember that the problem with ECB is that it is stateless and
    deterministic; the same 16 byte plaintext block will always
    produce the same 16 byte ciphertext. 

    */

    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;
    use rustc_serialize::hex::{FromHex};

    use strings::*;
    
    
    #[test]
    fn test_c08() {
        let expected = "d880619740a8a19b7840a8a31c810a3d08649af70dc06f4fd5d2d69c744cd283e2dd052f6b641dbf9d11b0348542bb5708649af70dc06f4fd5d2d69c744cd2839475c9dfdbc1d46597949d9c7e82bf5a08649af70dc06f4fd5d2d69c744cd28397a93eab8d6aecd566489154789a6b0308649af70dc06f4fd5d2d69c744cd283d403180c98c8f6db1f2a3f9c4040deb0ab51b29933f2c123c58386b06fba186a";
        let src = "data/8.txt";
        
	      let file = File::open(src).unwrap();
	      let reader = BufReader::new(file);
        
        let mut result:String = String::new();
        
	      for line in reader.lines() {
            let foo = line.unwrap();
            let l = foo.from_hex().unwrap();
            let count = repeat_count(&l, 16);
            if count > 1 {
                result = foo;
                //println!("{:?} {:?}", count, foo);
            }
	      }

        //println!("{:?}", result);
        assert_eq!(expected, result);
            
    }
}
