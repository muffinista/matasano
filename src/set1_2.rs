#[cfg(test)]
mod test {
    extern crate rustc_serialize;
    use rustc_serialize::hex::{FromHex, ToHex};
    
    #[test]
    fn test_c02() {
        let mut x = "1c0111001f010100061a024b53535009181c".from_hex().unwrap();
        let y = "686974207468652062756c6c277320657965".from_hex().unwrap();

        for i in 0..x.len() {
            x[i] ^= y[i];
        }
        
        let expected = "746865206b696420646f6e277420706c6179";

        let b = x.to_hex();
        assert_eq!(expected, b);
    }
}
