use std::collections::HashMap;

//pub fn cookie_to_hashmap<'a>(s: &'a str, result: &mut HashMap<&'a str, &'a str>) {
pub fn cookie_to_hashmap<'a>(s: &'a str, result: &mut HashMap<String, String>) {
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

//pub fn hashmap_to_cookie(data:HashMap<&str, &str>) -> &str {

//}


#[test]
fn test_cookie_to_hashmap() {
    let s = "foo=bar&baz=boo&boo=bim";
    let mut map:HashMap<String, String> = HashMap::new();

    cookie_to_hashmap(s, &mut map);

    assert_eq!(map["foo"], "bar");
    assert_eq!(map["baz"], "boo");
    assert_eq!(map["boo"], "bim");    

    // todo handle panic test
}
