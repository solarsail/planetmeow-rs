#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate dotenv;
extern crate chrono;
#[macro_use] extern crate lazy_static;

extern crate serde_json;
#[macro_use] extern crate serde_derive;


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
        .mount("/", routes![index,
               handlers::post::get_all,
               handlers::post::get,
               handlers::post::create,
               handlers::post::publish,
               handlers::post::update,
               handlers::post::delete,
               handlers::visitor::get_all,
               handlers::visitor::get,
               handlers::visitor::create,
               handlers::visitor::update,
               handlers::visitor::delete,
               ])
        .catch(errors![handlers::errors::not_found])
        .launch();
}
