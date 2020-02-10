use std::collections::HashMap;
use serde_json::{Value};

type Header<'a> = HashMap<&'a str, &'a str>;

pub struct HttpClient<'a> {
    headers: Box<Header<'a>>,
    base_url: &'a str
}

impl<'a> HttpClient<'a> {
    pub fn new(headers: Box<Header<'a>>, base_url: &'a str) -> HttpClient<'a> {
        HttpClient {
            headers,
            base_url
        }
    }
    pub async fn get(&self, url: &str) -> Result<Value, Box<dyn std::error::Error>> {
        let mut client = reqwest::Client::new().get(&format!("{}{}", self.base_url, url));
        for (name, value) in &*self.headers {
            client = client.header(*name, *value);
        }
        let resp = client.send().await?.json::<Value>().await?;
        Ok(resp)
    }
}