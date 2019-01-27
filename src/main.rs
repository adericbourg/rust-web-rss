#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate rocket_contrib;

#[cfg(test)]
mod index_tests;

use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct IndexContext {
    title: &'static str,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", &IndexContext { title: "Hello!" })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index])
}

fn main() {
    rocket().launch();
}
