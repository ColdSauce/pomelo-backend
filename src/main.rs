#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde;

extern crate iron;
extern crate router;
extern crate staticfile;
extern crate bodyparser;
extern crate mount;

use iron::prelude::*;
use router::Router;
use mount::Mount;
use iron::status;
use staticfile::Static;
use std::string::String;
use std::path::Path;
use std::vec::Vec;

mod models;
use models::{ClientInput, Category, Categories};


fn get_categories(links: Vec<String>) -> Vec<Category> {
    vec![Category { name: String::from("tech"), urls: links }]
}

fn categorize(input: ClientInput) -> String {
    let urls = input.urls.into_iter().map(|x| String::from(x)).collect();
    let cats = get_categories(urls);
    let result = Categories { results: cats };

    serde_json::to_string(&result).unwrap()
}

fn api(r: &mut Request) -> IronResult<Response> {
    let body = r.get::<bodyparser::Struct<ClientInput>>();
    match body {
        Ok(Some(struct_body)) => Ok(Response::with((status::Ok, categorize(struct_body)))),
        Ok(None) =>  Ok(Response::with((status::BadRequest, "No data sent"))),
        Err(_) =>  Ok(Response::with((status::BadRequest, "Invalid input"))),
    }
}

fn main() {
    let mut router = Router::new();
    let mut mount  = Mount::new();
    let indexfile = Static::new(Path::new("public/index.html"));
    let images = Static::new(Path::new("public/images"));

    router.post("/", api, "api");

    mount.mount("/api", router)
         .mount("/images/", images)
         .mount("/", indexfile);

    println!("Hello, world on 3000!");
    Iron::new(mount).http("localhost:3000").unwrap();
}
