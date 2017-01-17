#[cfg(test)]
mod test {
    extern crate rustc_serialize;
    use rustc_serialize::hex::FromHex;
    use rustc_serialize::base64::{ToBase64, STANDARD};
    
    #[test]
    fn test_c01() {
        let x = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let y = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let b = x.from_hex().unwrap().to_base64(STANDARD);
        assert_eq!(y, b);
    }
}
