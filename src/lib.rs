#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub struct UrlMatchResp {
    pub keys: HashMap<String, String>,
    pub is_matched: bool,
}

pub fn urlmatch(url: &str, pattern: &str
                    ) -> UrlMatchResp {
    lazy_static! {
        static ref RE_KEYS: Regex = Regex::new(r":(?P<key>[.a-zA-Z0-9_-]+)").unwrap();
    }
    let url_split: Vec<&str> = url.split("?").collect();
    let pattern_full = &RE_KEYS.replace_all( &pattern, r"(?P<$key>[.a-zA-Z0-9_-]+)");
    let mut keys: HashMap<String, String> = HashMap::new();
    let re = Regex::new(&["^", pattern_full, "$"].concat()).unwrap();
    
    let caps = match re.captures(url_split[0]) {
        None => return UrlMatchResp {
            keys: keys,
            is_matched: false,
        },
        Some(r) => r,
    };

    for (index, key) in re.capture_names().enumerate() {
        if let (Some(k), Some(c)) = (key, caps.get(index)) {
            keys.insert(k.to_owned(), c.as_str().to_string());
        }
    }

    return UrlMatchResp {
        keys: keys,
        is_matched: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_urlmatch_correct_1() {
        let correct_rel_url = "/en/v_3.0?lang=en&nav=42#title";
        let correct_pattern = "/:lang/:version";
        let r = urlmatch(correct_rel_url, correct_pattern);
        assert!(r.is_matched);
        assert_eq!(r.keys.len(), 2);
        assert_eq!(r.keys.get("lang"), Some(&"en"));
        assert_eq!(r.keys.get("version"), Some(&"v_3.0"));
        assert!(r.keys.contains_key("lang"));
        assert!(!r.keys.contains_key("wrong_key"));
    }

    #[test]
    fn test_urlmatch_correct_2() {
        let correct_abs_url = "https://example.com/en/v_3.0?lang=en&nav=42#title";
        let correct_pattern = "https://example.com/:lang/:version";
        let r = urlmatch(correct_abs_url, correct_pattern);
        assert!(r.is_matched);
        assert_eq!(r.keys.len(), 2);
        assert_eq!(r.keys.get("lang"), Some(&"en"));
        assert_eq!(r.keys.get("version"), Some(&"v_3.0"));
        assert!(r.keys.contains_key("lang"));
        assert!(!r.keys.contains_key("wrong_key"));
    }

    #[test]
    fn test_urlmatch_correct_3() {
        let correct_abs_url = "https://example.com/en/v_3.0?lang=en&nav=42#title";
        let correct_pattern = ":protocol://:host/:lang/:version";
        let r = urlmatch(correct_abs_url, correct_pattern);
        assert!(r.is_matched);
        assert_eq!(r.keys.len(), 4);
        assert_eq!(r.keys.get("protocol"), Some(&"https"));
        assert_eq!(r.keys.get("host"), Some(&"example.com"));
        assert_eq!(r.keys.get("lang"), Some(&"en"));
        assert_eq!(r.keys.get("version"), Some(&"v_3.0"));
        assert!(r.keys.contains_key("lang"));
        assert!(!r.keys.contains_key("wrong_key"));
    }

    #[test]
    fn test_urlmatch_incorrect_1() {
        let url = "abc";
        let pattern = "/:lang/:version";
        let r = urlmatch(url, pattern);
        assert!(!r.is_matched);
    }

    #[test]
    fn test_urlmatch_incorrect_2() {
        let url = "/en?/v_3.0?lang=en&nav=42#";
        let pattern = "/:lang/:version";
        let r = urlmatch(url, pattern);
        assert!(!r.is_matched);
    }

    #[test]
    fn test_urlmatch_incorrect_3() {
        let url = "/en/:v_3.0?lang=en&nav=42#";
        let pattern = "/:lang/:version";
        let r = urlmatch(url, pattern);
        assert!(!r.is_matched);
    }
}
