#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate time;

mod handlers;
mod db;
mod models;
mod schema;


#[get("/")]
fn index() -> &'static str {
    "Hello rocket!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .catch(errors![handlers::errors::not_found])
        .launch();
}
