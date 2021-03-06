#[cfg(test)]
mod test {
    //
    // PKCS#7 padding validation
    //
    // Write a function that takes a plaintext, determines if it has
    // valid PKCS#7 padding, and strips the padding off.
    //
    // The string:
    //
    // "ICE ICE BABY\x04\x04\x04\x04"
    // ... has valid padding, and produces the result "ICE ICE BABY".
    //
    // The string:
    //
    // "ICE ICE BABY\x05\x05\x05\x05"
    // ... does not have valid padding, nor does:
    //
    // "ICE ICE BABY\x01\x02\x03\x04"
    //
    //
    // If you are writing in a language with exceptions, like Python
    // or Ruby, make your function throw an exception on bad padding.
    //

    use pkcs::*;
    
    #[test]
    fn test_c15() {
        let src = "ICE ICE BABY\x04\x04\x04\x04";
        let expected = "ICE ICE BABY";

        let unpadded = pkcs_unpad(src.as_bytes()).unwrap();
        let result = String::from_utf8(unpadded).unwrap();
        
        assert_eq!(expected, result);


        let src2 = "ICE ICE BABY\x05\x05\x05\x05";
        assert!(pkcs_unpad(src2.as_bytes()).is_err());
        
        let src3 = "ICE ICE BABY\x01\x02\x03\x04";
        assert!(pkcs_unpad(src3.as_bytes()).is_err());
    }

}
