extern crate rss;

use chrono::DateTime;
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
        published: format_date(i.pub_date()),
        media_url: i.enclosure().map(|encl| String::from(encl.url())),
    }).collect();
    let podcast = Podcast {
        name: channel_name,
        items: podcast_items,
    };
    podcast
}

fn format_date(maybe_date: Option<&str>) -> Option<String> {
    maybe_date.map(|d| DateTime::parse_from_rfc2822(d).expect("Failed parsing published date"))
        .map(|date| date.format("%Y-%m-%d").to_string())
}
