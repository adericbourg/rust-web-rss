#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::env;
use std::fs;

use rocket_contrib::templates::Template;

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
    let configuration = load_configuration(configuration_file);
    fetch_data(configuration.subscriptions);
    rocket().launch();
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Configuration {
    pub subscriptions: Vec<Subscription>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Subscription {
    pub name: String,
    pub url: String,
}

pub fn load_configuration(configuration_file: &str) -> Configuration {
    println!("Loading configuration file: '{}'", configuration_file);
    let file_content = read_file(configuration_file);
    let configuration: Configuration = serde_yaml::from_str(&file_content).unwrap();

    configuration
}

fn read_file(file_name: &str) -> String {
    let contents = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    contents
}

fn fetch_data(sources: Vec<Subscription>) {
    for source in sources.iter() {
        println!("Fetching RSS for '{}' ({})", source.name, source.url);
    }
}
