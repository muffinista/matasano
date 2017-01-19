
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

/**
    Write a function to compute the edit distance/Hamming distance between two strings. The Hamming distance is just the number of differing bits. The distance between:
    this is a test
    and
    wokka wokka!!!
    is 37. Make sure your code agrees before you proceed.
*/
pub fn hamming_dist(s1: &str, s2: &str) -> usize {
    let x = s1.bytes().zip(s2.bytes());
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

    assert_eq!(37, hamming_dist(s1, s2));
}
