use std::collections::BTreeMap;

//pub fn cookie_to_hashmap<'a>(s: &'a str, result: &mut BTreeMap<&'a str, &'a str>) {
pub fn cookie_to_hashmap<'a>(s: &'a str, result: &mut BTreeMap<String, String>) {
    for k_v in s.split('&') {
        let mut tmp = k_v.split('=');
        //println!("{:?}", tmp);
        match (tmp.next(), tmp.next()) {
            (Some(k), Some(v)) => {
                result.insert(String::from(k), String::from(v));},
            _ => panic!("Invalid input"),
        }
    }
    println!("{:?}", result);
}



#[test]
fn test_cookie_to_hashmap() {
    let s = "foo=bar&baz=boo&boo=bim";
    let mut map:BTreeMap<String, String> = BTreeMap::new();

    cookie_to_hashmap(s, &mut map);

    assert_eq!(map["foo"], "bar");
    assert_eq!(map["baz"], "boo");
    assert_eq!(map["boo"], "bim");    

    // todo handle panic test
}

// your function should not allow encoding metacharacters (& and =).
// Eat them, quote them, whatever you want to do, but don't let people
// set their email address to "foo@bar.com&role=admin".
pub fn hashmap_to_cookie(src: BTreeMap<String, String>) -> String {
    let x:Vec<String> = src.iter().map(|(k, v)| [k.clone(), v.clone()].join("=")).collect();
    //println!("{:?}", x);
    
    let y:String = x.join("&");
    //println!("{:?}", y);

    y
}


#[test]
fn test_hashmap_to_cookie() {
    let expected = "baz=boo&boo=bim&foo=bar";

    let mut test: BTreeMap<String, String> = BTreeMap::new();
    test.insert("foo".to_string(), "bar".to_string() );
    test.insert("baz".to_string(), "boo".to_string() );
    test.insert("boo".to_string(), "bim".to_string() );
    
    let result = hashmap_to_cookie(test);
    assert_eq!(expected, result);
}
