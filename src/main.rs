#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/about")]
fn about() -> &'static str {
    "This website does absolutely nothing important."
}

fn main() {
    rocket::ignite().mount("/", routes![index, about]).launch();
}
