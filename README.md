urlmatch - a tiny library for url pattern matching. It can be useful in your url dispatcher.

Example usage:
```
use futures_lite::{future};
use urlmatch::urlmatch;

fn main() {
    future::block_on(amain());
}

async fn amain() {
    let r = urlmatch("https://example.com/en/v_3.0/", "https://example.com/:lang/:version/");
    println!("keys: {:?}", r.keys);
    println!("lang: {}", r.keys["lang"]);

    let url = "/en/v_3.0?lang=en&nav=42#title";
    let pattern = "/:lang/:version";
    let r = urlmatch(url, pattern);
    assert_eq!(r.is_matched, true);
    assert_eq!(r.keys.len(), 2);
    assert_eq!(r.keys.get("lang"), Some(&"en".to_string()));
    assert_eq!(r.keys.get("version"), Some(&"v_3.0".to_string()));
}

```

See https://github.com/SergeiMinaev/urlmatch/tree/master/examples for simple url dispatcher example.

Run example with:
```cargo run --example simple-url-dispatcher```
