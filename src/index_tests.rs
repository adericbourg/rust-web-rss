use super::{rocket, IndexContext};

use rocket::http::Method::*;
use rocket::http::Status;
use rocket::local::{Client, LocalResponse};
use rocket_contrib::templates::Template;

macro_rules! dispatch {
    ($method:expr, $path:expr, $test_fn:expr) => {{
        let client = Client::new(rocket()).unwrap();
        $test_fn(&client, client.req($method, $path).dispatch());
    }};
}

#[test]
fn test_index() {
    dispatch!(Get, "/", |client: &Client, mut response: LocalResponse| {
        let context = IndexContext { title: "Hello!" };

        let expected = Template::show(client.rocket(), "index", &context).unwrap();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some(expected));
    });
}
