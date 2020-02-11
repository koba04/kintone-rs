use std::collections::HashMap;
use url::{Url};
use serde_json::{Value};

type Header<'a> = HashMap<&'a str, &'a str>;

pub struct HttpClient<'a> {
    headers: Box<Header<'a>>,
    base_url: &'a str
}

pub type Params<'a> = Vec<(&'a str, String)>;

impl<'a> HttpClient<'a> {
    pub fn new(headers: Box<Header<'a>>, base_url: &'a str) -> HttpClient<'a> {
        HttpClient {
            headers,
            base_url
        }
    }
    pub async fn get(&self, path: &str, params: &Params<'_>) -> Result<Value, Box<dyn std::error::Error>> {
        let url = self.build_url(path, params);
        let mut client = reqwest::Client::new().get(&url);
        for (name, value) in &*self.headers {
            client = client.header(*name, *value);
        }
        let resp = client.send().await?.json::<Value>().await?;
        Ok(resp)
    }
    fn build_url(&self, end_point: &str, params: &Params) -> String {
        let mut url = Url::parse(self.base_url).expect("invalid base_url");
        url.set_path(&format!("k/v1/{}", end_point));
        for (key, value) in params {
            url.query_pairs_mut().append_pair(key, value);
        }
        println!("request url is {}", url.as_str());
        String::from(url.as_str())
    }
}