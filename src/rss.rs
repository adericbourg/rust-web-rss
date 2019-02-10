extern crate reqwest;

use crate::configuration;

pub struct Podcast {
    pub name: String,
    pub items: Vec<PodcastItem>,
}

pub struct PodcastItem {
    title: String,
    // TODO parse date
    published: String,
    media_url: String,
}

pub fn fetch_data(subscriptions: Vec<configuration::Subscription>) -> Vec<Podcast> {
    for subscription in subscriptions.iter() {
        fetch(subscription);
    }
    vec!()
}

fn fetch(subscription: &configuration::Subscription) {
    let mut response = reqwest::get(&subscription.url).expect("Failed to fetch subscription");
    let text = response.text().expect("Failed to get response content");
    println!("{:?}", text);
}