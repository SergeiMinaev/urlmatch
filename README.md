urlmatch - a micro library for url pattern matching. It can be useful in your url dispatcher.

Example usage:
```
use urlmatch::urlmatch;

fn main() {
    let r = urlmatch("https://example.com/en/v_3.0/",
                     "https://example.com/:lang/:version/");
    println!("keys: {:?}", r);
    println!("lang: {}", r["lang"]);
}
```
