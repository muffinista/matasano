
pub fn xor_encode(src: &[u8], key: &[u8]) -> Vec<u8> {
    let mut out:Vec<u8> = Vec::new();
    
    let mut iter = key.iter().cycle();
    for i in 0..src.len() {
        out.push(src[i] ^ iter.next().unwrap());
    }

    out
}
    
