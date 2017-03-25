extern crate rand;
use rand::{thread_rng, Rng};

use crypto::{encrypt_cbc, encrypt_ecb_chunk};

/// generate a random ascii key of the specified size
/// @todo might be fun to redo this in pure rust (no deps)
pub fn random_key(size: usize) -> String {
    let s: String = thread_rng().gen_ascii_chars().take(size).collect();

    s
}

#[test]
fn test_random_key() {
    let len = 16;
    assert_eq!(random_key(len).len(), len);
}


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
// NOTE: I return a tuple with the ciphertext and a boolean that
// indicates if we used CBC if true, or ECB if false. This way we
// can test that our detection actually works
//
pub fn encryption_oracle(plaintext:&[u8]) -> (Vec<u8>, bool) {
    let key = random_key(16);

    let prefix_size = rand::random::<usize>() % 5 + 5;
    let postfix_size = rand::random::<usize>() % 5 + 5;    

    let prefix: String = thread_rng().gen_ascii_chars().take(prefix_size).collect();
    let postfix: String = thread_rng().gen_ascii_chars().take(postfix_size).collect();    

    let work:Vec<u8> = [
        (&prefix).as_bytes().to_vec(),
        plaintext.to_vec(),
        (&postfix).as_bytes().to_vec()
    ].concat();

    let mut rng = rand::thread_rng();
    let result:Vec<u8>;

    let mut used_cbc:bool = true;

    if rng.gen() { // random bool
        //println!("I used CBC!");
        result = encrypt_cbc(&work, &key, key.len());
    }
    else {
        //println!("I used ECB!");
        used_cbc = false;
        result = encrypt_ecb_chunk(work, &key);
    }
    
    (result, used_cbc)
}