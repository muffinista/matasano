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

    // Encrypt the encoded user profile under the key; "provide" that to the "attacker".
    // Decrypt the encoded user profile and parse it.

    // Using only the user input to profile_for() (as an oracle to
    // generate "valid" ciphertexts) and the ciphertexts themselves,
    // make a role=admin profile.
 
    use std::collections::BTreeMap;
    use cookies::*;
    
    fn profile_for(email:&str) -> String {
        // strip & and =
        let clean_email = email.replace("&", "").replace("=", "");

        let mut data: BTreeMap<String, String> = BTreeMap::new();
        data.insert("email".to_string(), clean_email.to_string() );
        data.insert("uid".to_string(), "10".to_string() );
        data.insert("role".to_string(), "user".to_string() );

        hashmap_to_cookie(data)
    }
    
    #[test]
    fn test_c13() {
        let email = "foo@bar.com";
        let cookie = profile_for(email);
        println!("{:?}", cookie);

        assert_eq!("email=foo@bar.com&role=user&uid=10", cookie);
    }
}
