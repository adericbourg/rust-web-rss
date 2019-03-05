extern crate rss;

use rss::Channel;

use crate::configuration;

#[derive(Debug)]
#[derive(Serialize)]
pub struct Podcast {
    pub name: String,
    pub items: Vec<PodcastItem>,
}

#[derive(Debug)]
#[derive(Serialize)]
pub struct PodcastItem {
    title: Option<String>,
    // TODO parse date
    published: Option<String>,
    media_url: Option<String>,
}

pub fn fetch_podcasts(subscriptions: Vec<configuration::Subscription>) -> Vec<Podcast> {
    subscriptions.iter().map(fetch).collect()
}

fn fetch(subscription: &configuration::Subscription) -> Podcast {
    let channel = Channel::from_url(&subscription.url).expect("Failed to fetch RSS feed");
    let channel_name = String::from(channel.title());
    let podcast_items: Vec<PodcastItem> = channel.into_items().into_iter().map(|i| PodcastItem {
        title: i.title().map(|t| String::from(t)),
        published: i.pub_date().map(|pd| String::from(pd)),
        media_url: i.enclosure().map(|encl| String::from(encl.url())),
    }).collect();
    let podcast = Podcast {
        name: channel_name,
        items: podcast_items,
    };
    podcast
}