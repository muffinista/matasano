#[cfg(test)]
mod test {

    // An ECB/CBC detection oracle
    // Now that you have ECB and CBC working:
    //
    // Write a function to generate a random AES key; that's just 16
    // random bytes.
    //
    // Write a function that encrypts data under an unknown key --- that
    // is, a function that generates a random key and encrypts under it.
    //
    // The function should look like:
    //
    // encryption_oracle(your-input)
    // => [MEANINGLESS JIBBER JABBER]
    // Under the hood, have the function append 5-10 bytes (count chosen
    // randomly) before the plaintext and 5-10 bytes after the plaintext.
    //
    // Now, have the function choose to encrypt under ECB 1/2 the time,
    // and under CBC the other half (just use random IVs each time for
    // CBC). Use rand(2) to decide which to use.
    //
    // Detect the block cipher mode the function is using each time. You
    // should end up with a piece of code that, pointed at a block box
    // that might be encrypting ECB or CBC, tells you which one is
    // happening.
    
    use oracle::*;
    use strings::*;
    use std::iter;

    #[test]
    fn test_c11() {
        // in rust 1.16, we can use str::repeat and do "x".repeat(256)
        let s: String = iter::repeat("x").take(256).collect();

        // run a bunch of tests to make sure we aren't lucky
        for _ in 1..100 {
            let (x, used_cbc) = encryption_oracle(s.as_bytes());

            //println!("{:?}", x);

            let count = repeat_count(&x, 16);
            if count > 1 {
                //println!("ECB detected, {} used", used_cbc);
                assert!(!used_cbc);
            }
            else {
                //println!("CBC detected, {} used", used_cbc);
                assert!(used_cbc);
            }
        }
    }
}
