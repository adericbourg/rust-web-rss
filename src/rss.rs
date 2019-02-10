use crate::configuration;

pub fn fetch_data(sources: Vec<configuration::Subscription>) {
    for source in sources.iter() {
        println!("Fetching RSS for '{}' ({})", source.name, source.url);
    }
}
