use std::collections::HashMap;
use futures_lite::{future, Future, FutureExt};
use urlmatch::urlmatch;


struct Path<Fut>
where
    Fut: Future<Output = String>,
{
    p: &'static str,
    f: fn(HashMap<String, String>) -> Fut,
}


async fn url_dispatcher(url: String) -> String {
    let routes = vec![
        Path {p: &"/", f: |args| home(args).boxed()},
        Path {p: &"/profile", f: |args| profile(args).boxed()},
        Path {p: &"/catalogue/:ctg/:id", f: |args| catalogue(args).boxed()},
        Path {p: &"/json", f: |args| get_json(args).boxed()},
    ];
    for route in routes.iter() {
        let r = urlmatch(&url, route.p);
        if r.is_matched {
            return (route.f)(r.keys).await;
        }
    }
    "Not found".to_string()
}

async fn home(args: HashMap<String, String>) -> String {
    return format!("home(), args: {args:?}")
}
async fn catalogue(args: HashMap<String, String>) -> String {
    return format!("catalogue(), args: {:?}", args);
}
async fn profile(args: HashMap<String, String>) -> String {
    return format!("profile(), args: {:?}", args)
}
async fn get_json(_args: HashMap<String, String>) -> String {
    return format!(r#"
        {{
            "name": "Adam",
            "age": "{}"
        }}
    "#, i64::MIN)
}

fn main() {
    future::block_on(amain());
}

async fn amain() {
    let r = urlmatch("https://example.com/en/v_3.0/", "https://example.com/:lang/:version/");
    println!("keys: {:?}", r.keys);
    println!("lang: {}", r.keys["lang"]);

    let r = url_dispatcher("/catalogue/books/123".to_string()).await;
    println!("Result: {r}");
    
    let not_found = url_dispatcher("/wrong/url".to_string()).await;
    println!("Result: {not_found}");

    let url = "/en/v_3.0?lang=en&nav=42#title";
    let pattern = "/:lang/:version";
    let r = urlmatch(url, pattern);
    assert_eq!(r.is_matched, true);
    assert_eq!(r.keys.len(), 2);
    assert_eq!(r.keys.get("lang"), Some(&"en".to_string()));
    assert_eq!(r.keys.get("version"), Some(&"v_3.0".to_string()));
}
