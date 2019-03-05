#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::env;

use rocket::State;
use rocket_contrib::templates::Template;

use crate::podcast::Podcast;

mod configuration;
mod podcast;

#[cfg(test)]
mod index_tests;


#[derive(Serialize)]
struct IndexContext<'a> {
    title: &'static str,
    podcasts: &'a Vec<Podcast>,
}

#[get("/")]
fn index(podcasts: State<Vec<Podcast>>) -> Template {
    Template::render("index", &IndexContext {
        title: "Podcasts",
        podcasts: podcasts.inner(),
    })
}

fn rocket(podcasts: Vec<Podcast>) -> rocket::Rocket {
    rocket::ignite()
        .manage(podcasts)
        .attach(Template::fairing())
        .mount("/", routes![index])
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Missing configuration file as first argument");
        ::std::process::exit(1);
    }
    let configuration_file = &args[1];
    let configuration = configuration::load(configuration_file);

    let podcasts = podcast::fetch_podcasts(configuration.subscriptions);
    rocket(podcasts).launch();
}
