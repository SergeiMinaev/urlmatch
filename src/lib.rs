#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
use url::Url;

// draft
pub fn urlmatch() {
    let url = "http://example.com/en/3.0";
    lazy_static! {
        static ref KEY: Regex = Regex::new(r"(?P<key>[?&.])").unwrap();
        static ref RE: Regex = Regex::new(r":(?P<key>[.a-zA-Z0-9_-]+)").unwrap();
    }
    let pattern_0 = "http://example.com/:lang/:version";
    let pattern_1 = String::from(KEY.replace_all(&pattern_0, r"\$key"));
    println!("pattern_0: {}", pattern_0);
    println!("pattern_1: {}", pattern_1);
    let mut hashmap: HashMap<String, &str> = HashMap::new();
    let mut expr = String::from(r"^");
    expr.push_str(&RE.replace_all( &pattern_1, r"(?P<$key>[.a-zA-Z0-9_-]+)"));
    expr.push_str("$");
    let re = Regex::new(&expr).unwrap();
    
    let caps = re.captures(url).unwrap();
    for (index, key) in re.capture_names().enumerate() {
        if let (Some(k), Some(c)) = (key, caps.get(index)) {
            hashmap.insert(k.to_owned(), c.as_str());
        }
    }

    println!("R {:?}", hashmap);
    println!("R {:?}", hashmap["version"]);
    
    let url2 = Url::parse("https://example.net?lang=ru&nav=123#lala").unwrap();
    let pairs: HashMap<_, _> = url2.query_pairs().into_owned().collect();
    println!("pairs: {:?}", pairs);
    println!("pairs[lang]: {}", pairs["lang"]);
    
}
