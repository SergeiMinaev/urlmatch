urlmatch - a micro library for url pattern matching. It can be useful in your url dispatcher.

Example usage:
```
use urlmatch::urlmatch;

fn main() {
    let r = urlmatch("https://example.com/en/v_3.0/",
                     "https://example.com/:lang/:version/");
    println!("keys: {:?}", r.keys);
    println!("lang: {}", r.keys["lang"]);
}
```

Simple url router example:
```
use urlmatch::urlmatch;
use std::collections::HashMap;

struct Path {
    p: &'static str,
    f: fn(&HashMap<String, &str>),
}

fn main() {
    let url = "/catalogue/somectg/100/";
    url_dispatcher(&url);
}

fn url_dispatcher(url: &str) {
    let routes = vec![
        Path {p: &"/", f: home},
        Path {p: &"/catalogue/:ctg/:id/", f: catalogue},
        Path {p: &"/profile/:username/", f: profile},
    ];
    for route in routes.iter() {
        let r = urlmatch(url, route.p);
        if r.is_matched {
            (route.f)(&r);
            break;
        }
    }
}

fn home(args: &HashMap<String, &str>) {
    println!("home(), args: {:?}", args);
}
fn catalogue(args: &HashMap<String, &str>) {
    println!("catalogue(), args: {:?}", args);
}
fn profile(args: &HashMap<String, &str>) {
    println!("profile(), args: {:?}", args);
}
```
