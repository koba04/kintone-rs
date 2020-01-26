use std::collections::HashMap;
use serde_json::{Value};

pub struct HttpClient<'a> {
    headers: &'a HashMap<String, String>
}

impl<'a> HttpClient<'a> {
    pub fn new(headers: &'a HashMap<String, String>) -> HttpClient<'a> {
        HttpClient {
            headers
        }
    }
    pub async fn get(self, url: String) -> Result<Value, Box<dyn std::error::Error>> {
        let mut client = reqwest::Client::new().get(&url);
        for (name, value) in self.headers {
            client = client.header(name, value);
        }
        let resp = client
        .send()
        .await?
        .json::<Value>()
        .await?;
        // TODO: should not clone
        Ok(resp["record"].clone())
    }
}