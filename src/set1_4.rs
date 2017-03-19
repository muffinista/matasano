#[cfg(test)]
mod test {

    use std::str;
    
    /**

    Detect single-character XOR
    One of the 60-character strings in this file has been encrypted by single-character XOR.

    Find it.

    (Your code from #3 should help.)

    */


    
    use std::fs::File;
    use std::io::{BufReader};
    use std::io::prelude::*;
    use rustc_serialize::hex::{FromHex};

    use scores;
    use decode;
    
    
    #[test]
    fn test_c04() {

        let f = File::open("data/4.txt").unwrap();
        let f  = BufReader::new(f);

        let mut lines_max = 0;
        let mut lines_result = String::new();

        for line in f.lines() {
            let src = line.unwrap();
            
            let mut max = 0;
            let mut result: String = String::new();

            for char in 0..255 {
                let foo = decode::xor_decode(&src.from_hex().unwrap(), char);
                let val = scores::string_score(&foo);
            
                if val > max {
                    max = val;
                    result = foo.to_string();
                }
            }
            
            if max > lines_max {
                lines_max = max;
                lines_result = result;
            }
            
            //println!("{} {}", lines_max, lines_result);
        }

        assert_eq!("Now that the party is jumping\n", lines_result);
    }
}
