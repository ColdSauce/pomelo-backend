#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde;
extern crate iron;
extern crate router;
use iron::prelude::*;
use router::Router;
use iron::status;


#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn index(_: &mut Request) -> IronResult<Response> {
    let point = Point { x: 1, y: 2 };
    let serialized = serde_json::to_string(&point).unwrap();
    Ok(Response::with((status::Ok,  serialized))) 
}

fn main() {
    
    let mut router = Router::new();
    router.get("/", index, "index");

    println!("Hello, world on 3000!");
    Iron::new(router).http("localhost:3000").unwrap();
}
