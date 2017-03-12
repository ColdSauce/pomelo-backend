extern crate iron;
extern crate router;
use iron::prelude::*;
use router::Router;
use iron::status;

fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello world!"))) 
}

fn main() {
    
    let mut router = Router::new();
    router.get("/", index, "index");

    println!("Hello, world on 3000!");
    Iron::new(router).http("localhost:3000").unwrap();
}
