extern crate iron;
extern crate mount;
#[macro_use] extern crate juniper;

use juniper::iron_handlers::{GraphQLHandler, GraphiQLHandler};
use juniper::tests::model::Database;
use juniper::EmptyMutation;
use iron::prelude::*;
use iron::status;
use mount::Mount;


fn context_factory(_: &mut Request) -> Database {
    Database::new()
}

fn main() {
    let mut mount = Mount::new();
    let graphql_endpoint = GraphQLHandler::new(
        context_factory,
        Database::new(),
        EmptyMutation::<Database>::new(),
    );
    let graphiql_endpoint = GraphiQLHandler::new("/graphql");

    mount.mount("/", graphiql_endpoint);
    mount.mount("/graphql", graphql_endpoint);

    println!("Hello, world on 3000!");
    Iron::new(mount).http("localhost:3000").unwrap();
}
