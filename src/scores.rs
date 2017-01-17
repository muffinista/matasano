
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

