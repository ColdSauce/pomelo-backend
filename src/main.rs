extern crate iron;
use iron::prelude::*;
use iron::status;

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}


fn main() {
    let _server = Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("Hello, world on 3000!");
}
