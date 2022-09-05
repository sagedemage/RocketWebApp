#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rocket_contrib;

use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
    title: String
}

#[get("/")]
fn index() -> Template {
    let context = TemplateContext{ title: String::from("Home") };
    Template::render("includes/home", &context)
}

#[get("/about")]
fn about() -> Template {
    let context = TemplateContext{ title: String::from("About") };
    Template::render("includes/about", &context)
}

#[catch(404)]
fn page_not_found() -> Template {
    let context = TemplateContext{ title: String::from("404 Page Not Found") };
    Template::render("errors/404", &context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, about])
        .attach(Template::fairing())
        .register(catchers![page_not_found])
}

fn main() {
    rocket().launch();
}
