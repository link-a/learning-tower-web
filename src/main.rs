#[macro_use]
extern crate tower_web;
extern crate tokio;

use tower_web::ServiceBuilder;
use tokio::prelude::*;

#[derive(Clone, Debug)]
struct HelloWorld;

#[derive(Response)]
struct HelloResponse {
    message: &'static str,
}

impl_web! {
    impl HelloWorld {
        #[get("/")]
        #[content_type("json")]
        fn hello_world(&self) -> Result<HelloResponse, ()> {
            println!("responding to request");
            Ok(HelloResponse {
                message: "hello world",
            })
        }
    }
}

pub fn main() {
    let addr = "0.0.0.0:8000".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(HelloWorld)
        .run(&addr)
        .unwrap();
}