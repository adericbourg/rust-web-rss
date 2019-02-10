#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::env;

use rocket_contrib::templates::Template;

mod configuration;
mod rss;

#[cfg(test)]
mod index_tests;


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
    let args: Vec<String> = env::args().collect();
    let configuration_file = &args[1];
    let configuration = configuration::load(configuration_file);
    rss::fetch_data(configuration.subscriptions);
    rocket().launch();
}
