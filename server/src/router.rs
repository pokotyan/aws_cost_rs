#[macro_use]
use nickel::{HttpRouter, Nickel};
extern crate serde_json;

#[tokio::main]
async fn fetch() -> Result<String, reqwest::Error> {
    let res = reqwest::get("https://hyper.rs").await?;

    // println!("Status: {}", res.status());

    let body = res.text().await?;

    // println!("Body:\n\n{}", body);

    Ok(body)
}

pub fn explicit_router() -> nickel::Router {
    let mut router = Nickel::router();

    // Wildcard '*' routes are supported
    // - '*' will match a single directories seperated by '/'
    // - '**' will match potentially many directories

    // Single wildcard:
    // go to http://localhost:6767/some/crazy/route to see this route in action
    router.get(
        "/some/*/route",
        middleware! {
            let res = fetch().unwrap();

            // println!("{}", &format!(r#"{{ "res": {:?} }}"#, res));

            // let hoge = serde_json::from_str::<serde_json::Value>(&format!(r#"{{ "res": {:?} }}"#, res)).unwrap();

            // hoge
            // "This matches /some/crazy/route but not /some/super/crazy/route"
            format!(r#"{{ "res": {:?} }}"#, res)
        },
    );

    // Double wildcards:
    // go to http://localhost:6767/a/nice/route
    // or http://localhost:6767/a/super/nice/route to see this route in action
    router.get(
        "/a/**/route",
        middleware! {
            "This matches /a/crazy/route and also /a/super/crazy/route"
        },
    );

    router
}
