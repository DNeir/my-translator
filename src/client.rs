use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;

pub fn create_client() -> Result<Client, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "Accept-Encoding",
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    let key: &str = include_str!("key.txt").trim();
    headers.insert("X-RapidAPI-Key", HeaderValue::from_static(key));
    headers.insert(
        "X-RapidAPI-Host",
        HeaderValue::from_static("text-translator2.p.rapidapi.com"),
    );

    Ok(Client::builder().default_headers(headers).build()?)
}
