use std::cmp::Ordering;

fn char_score(c:char) -> u32 {
    return match c.to_uppercase().next().unwrap() {
        'E' => 1251,
        'T' => 925,
        'A' => 804,
        'O' => 760,
        'I' => 726,
        'N' => 709,
        'S' => 654,
        'R' => 612,
        'H' => 549,
        'L' => 414,
        'D' => 399,
        'C' => 306,
        'U' => 271,
        'M' => 253,
        'F' => 230,
        'P' => 200,
        'G' => 196,
        'W' => 192,
        'Y' => 173,
        'B' => 154,
        'V' => 99,
        'K' => 67,
        'X' => 19,
        'J' => 16,
        'Q' => 11,
        'Z' => 90,
        ' ' => 1000,
        '\n' => 1000,
        _ => 0
    }
}

// 126 Cooking MC's like a pound of bacon

pub fn string_score(s: &str) -> u32 {
    return s.chars().fold(0, |sum, x| sum + char_score(x));
}

#[test]
fn test_string_score() {
    let src = "ETA";
    assert_eq!(2980, string_score(src));
    
    let src2 = "";
    assert_eq!(0, string_score(src2));
}

/** Write a function to compute the edit distance/Hamming distance
    between two strings. The Hamming distance is just the number of
    differing bits.

    The distance between:

    this is a test
    and
    wokka wokka!!!

    is 37. Make sure your code agrees before you proceed.
*/
pub fn hamming_dist(s1: &[u8], s2: &[u8]) -> usize {
    let x = s1.iter().zip(s2.iter());
    let mut distance = 0;

    for (i, j) in x {
        let diff = i ^ j;
        if (diff & 0b00000001) > 0 { distance += 1; }
        if (diff & 0b00000010) > 0 { distance += 1; }
        if (diff & 0b00000100) > 0 { distance += 1; }
        if (diff & 0b00001000) > 0 { distance += 1; }
        if (diff & 0b00010000) > 0 { distance += 1; }
        if (diff & 0b00100000) > 0 { distance += 1; }
        if (diff & 0b01000000) > 0 { distance += 1; }
        if (diff & 0b10000000) > 0 { distance += 1; }
    }

    distance
}


#[test]
fn test_hamming_dist() {
    let s1 = "this is a test";
    let s2 = "wokka wokka!!!";

    assert_eq!(37, hamming_dist(s1.as_bytes(), s2.as_bytes()));
}

pub struct Dist {
    pub length: usize,
    pub dist: f64,
}

pub fn hamming_dist_for_keysizes(buffer: &[u8], min: usize, max: usize) -> Vec<Dist> {
    let mut scores: Vec<Dist> = Vec::new();
        
    for keysize_guess in min..max {
        let s1 = &buffer[0..keysize_guess];
        let s2 = &buffer[keysize_guess..keysize_guess*2];
        let s3 = &buffer[keysize_guess*2..keysize_guess*3];
        let s4 = &buffer[keysize_guess*3..keysize_guess*4];

        let dist:f64 = (
            hamming_dist(s1, s2) +
            hamming_dist(s1, s3) +
            hamming_dist(s1, s4) +            
            hamming_dist(s2, s3) +
            hamming_dist(s2, s4) +
            hamming_dist(s3, s4) ) as f64 / keysize_guess as f64 / 6.0;
            
        let d = Dist { length: keysize_guess, dist: dist };
            
        scores.push(d);
    }
    
    scores.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap_or(Ordering::Equal));

    scores
}