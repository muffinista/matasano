#[cfg(test)]
mod test {

    // ECB cut-and-paste
    // Write a k=v parsing routine, as if for a structured cookie. The routine should take:
    //
    // foo=bar&baz=qux&zap=zazzle
    //
    // ... and produce:
    //
    // {
    //   foo: 'bar',
    //   baz: 'qux',
    //   zap: 'zazzle'
    // }
    // (you know, the object; I don't care if you convert it to JSON).

    // Now write a function that encodes a user profile in that
    // format, given an email address. You should have something like:

    // profile_for("foo@bar.com")
    // ... and it should produce:

    // {
    //   email: 'foo@bar.com',
    //   uid: 10,
    //   role: 'user'
    // }
    //
    // ... encoded as:
    //
    // email=foo@bar.com&uid=10&role=user

    // Your "profile_for" function should not allow encoding
    // metacharacters (& and =). Eat them, quote them, whatever you
    // want to do, but don't let people set their email address to
    // "foo@bar.com&role=admin".

    // Now, two more easy functions. Generate a random AES key, then:

 
    use std::collections::BTreeMap;
    use cookies::*;
    use crypto::*;
    use oracle::*;

    struct Oracle {
        key: String
    }

    impl Oracle {
        fn new() -> Oracle {
            Oracle {
                // we aren't supposed to know the value of this key!
                key: random_key(16)
            }
        }
        fn respond(&self, plaintext:&[u8]) -> Vec<u8> {
            let work:Vec<u8> = plaintext.to_vec();
            
            let result:Vec<u8> = encrypt_ecb_chunk(work, &self.key);
            result
        }

        fn decrypt(&self, ciphertext:&[u8]) -> Vec<u8> {
            decrypt_ecb_chunk(ciphertext.to_vec(), self.key.as_str())
        }
    }
    
    fn profile_for(email:&str) -> String {
        // strip & and =
        let clean_email = email.replace("&", "").replace("=", "");

        let mut data: BTreeMap<String, String> = BTreeMap::new();
        data.insert("email".to_string(), clean_email.to_string() );

        // note -- the challenge has you use uid instead of id_ as a key,
        // but i'm lazy and the BTreeMap returns data in alphabetical order
        data.insert("id_".to_string(), "10".to_string() );
        data.insert("role".to_string(), "user".to_string() );

        hashmap_to_cookie(data)
    }
    
    #[test]
    fn test_c13() {
        let oracle = Oracle::new();

        let email = "foo@bar.com";
        let cookie = profile_for(email);
        //println!("{:?}", cookie);

        assert_eq!("email=foo@bar.com&id_=10&role=user", cookie);

        // Encrypt the encoded user profile under the key; "provide" that to the "attacker".

        // Decrypt the encoded user profile and parse it.

        // Using only the user input to profile_for() (as an oracle to
        // generate "valid" ciphertexts) and the ciphertexts themselves,
        // make a role=admin profile.


        // this puts the role right on a boundary
        // ie .....role=|value
        let email = "AAAAAAAAAAAAA";
        let cookie = profile_for(email);

        let base = oracle.respond(cookie.as_bytes());
        // println!("{}", cookie);
        // println!("12345678901234567890123456789012345678901234567890123456789");
        // println!("\n{}", &cookie[0..32]);

        // email=AAAAAAAAAAAAA&id_=10&role=user
        // 12345678901234567890123456789012345678901234567890123456789
        // email=AAAAAAAAAAAAA&id_=10&role=

        // generate a fake email address to get
        // the bytes for 'admin'
        let admin_email = "AAAAAAAAAAadmin";
        let admin_cookie = profile_for(admin_email);

        //println!("\nadmin\n{}", &admin_cookie[16..32]);

        let admin_base = oracle.respond(admin_cookie.as_bytes());
        //println!("{:?}", admin_base);

        let start = &base[0..32];
        let admin = &admin_base[16..32];

        let padding_cookie = profile_for("AAAAAAAAAAAAAA");
        let padding_base = oracle.respond(padding_cookie.as_bytes());

        let padding = &padding_base[32..48];

        let mut output = Vec::new();
        output.extend_from_slice(&start);
        output.extend_from_slice(&admin);
        output.extend_from_slice(&padding);
        
        let y2 = oracle.decrypt(&output);
        let output2 = String::from_utf8(y2).unwrap();

        assert_eq!("email=AAAAAAAAAAAAA&id_=10&role=admin&id_=10&rol=user", output2);
    }
}
