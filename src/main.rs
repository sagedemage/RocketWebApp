
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/about")]
fn about() -> &'static str {
    "This website does absolutely nothing important."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, about])
}
