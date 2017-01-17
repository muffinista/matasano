#[cfg(test)]
mod test {

    use std::str;
    
    /**
    * Single-byte XOR cipher
    
    The hex encoded string:

      1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736

    ... has been XOR'd against a single character. Find the key, decrypt the message.

    You can do this by hand. But don't: write code to do it for you.

    How? Devise some method for "scoring" a piece of English
    plaintext. Character frequency is a good metric. Evaluate each
    output and choose the one with the best score.

    */
    

    extern crate rustc_serialize;
    use rustc_serialize::hex::{FromHex};


    use scores;
    
    fn decode(x: &str, c: u8) -> String {
        //let mut out = String::new();

        let mut out = x.from_hex().unwrap();

        for i in 0..out.len() {
            out[i] ^= c;
        }

        let result = String::from_utf8(out);

        if ! result.is_err() {
            result.unwrap()
        }
        else {
            String::new()
        }
    }

    
    
    
    #[test]
    fn test_c03() {
        let src = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let expected = "Cooking MC's like a pound of bacon";

        
        let mut max = 0;
        let mut result: String = String::new();

        for char in 0..255 {
            let foo = decode(src, char);
            let val = scores::string_score(&foo);
            
            if val > max {
                max = val;
                result = foo.to_string();
            }
            //println!("{} {}", val, foo);
        }

        println!("{} {}", max, &result);
        assert_eq!(&result, expected);
            
    }
}
