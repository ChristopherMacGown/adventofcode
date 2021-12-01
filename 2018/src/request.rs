use assets;
use failure;
use reqwest::{header, Client};

use std::str;

pub type Error = failure::Error;

pub fn get<T: reqwest::IntoUrl>(url: T) -> reqwest::Result<reqwest::Response> {
    let cookie = assets::get("session.cookie").unwrap();
    let session = str::from_utf8(&cookie).unwrap().trim_right().to_string();

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::COOKIE,
        header::HeaderValue::from_str(&session).unwrap(),
    );
    let client = Client::builder().default_headers(headers).build()?;

    client.get(url).send()
}
