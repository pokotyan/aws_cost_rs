#[macro_use]
extern crate nickel;
extern crate hyper;

use hyper::Method;
use nickel::{HttpRouter, Nickel};

mod router;

fn main() {
    let mut server = Nickel::new();

    // Nickel provides a default router on the server for getting
    // up and running quickly. If you want to partition out your app
    // you might want to use an explicit router though.
    server.utilize(router::explicit_router());

    // Most common HTTP verbs are extension methods added from the HttpRouter trait.
    // You can see other examples such as 'json' to see other verbs in use.
    // For other HTTP verbs, you can use the `add_route` method.
    // go to http://localhost:6767/foo to see this route in action
    server.get(
        "/:foo",
        middleware! { |request|
            let foo = request.param("foo").unwrap();
            let format = request.param("format").unwrap();
            format!("Foo is '{}'. The requested format is '{}'", foo, format)
        },
    );

    server.listen("127.0.0.1:6767").unwrap();
}
