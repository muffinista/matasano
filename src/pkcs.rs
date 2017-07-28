
pub fn pkcs_pad(s: &str, size:usize) -> String {
	  let mut output:Vec<u8> = s.as_bytes().clone().to_vec();

    let c = (size - (output.len() % size)) as u8;

    while output.len() % size != 0 {
        output.push(c);
    }
    String::from_utf8(output).unwrap()
}

//
// determine if a string has valid PKCS padding
//
pub fn pkcs_validate(s: &[u8]) -> bool {
    // get the last character in the string
    let last = s.last();

    if last.is_none() {
        return false;
    }

    let padchar:u8 = *last.unwrap();
    
    
    if padchar as usize > s.len() {
        return false;
    }

    s.iter()
        .skip(s.len() - padchar as usize)
        .all(|x: &u8| *x == padchar as u8)
    
}

pub fn pkcs_unpad(s: &[u8]) -> Option<Vec<u8>> {
    if ! pkcs_validate(s) {
        return None;
    }

    let last:usize = *s.last().unwrap()  as usize;
    let mut output = s.to_vec();

    output.truncate(s.len() - last);
    Some(output)
       
}

#[test]
fn test_valid_pkcs_unpad() {
    let src = "YELLOW SUBMARINE\x04\x04\x04\x04";
    let expected = "YELLOW SUBMARINE";
    
    let unpadded = pkcs_unpad(src.as_bytes()).unwrap();
    let result = String::from_utf8(unpadded).unwrap();
    
    assert_eq!(expected, result);
}

#[test]
fn test_invalid_pkcs_unpad() {
    let src = "YELLOW SUBMARINE\x04\x04\x04\x05";
    let expected = None;
    
    let unpadded = pkcs_unpad(src.as_bytes());
    assert_eq!(expected, unpadded);
}


