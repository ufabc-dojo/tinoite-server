#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

pub mod models;
pub mod controllers;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/user", routes![controllers::users::create])
        .launch();
}
