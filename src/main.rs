#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod handlers;

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
