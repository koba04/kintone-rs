use serde_json::{Value};
use std::collections::HashMap;
use crate::http::HttpClient;

pub struct Kintone<'a> {
    base_url: &'a str,
    api_token: &'a str
}

impl<'a> Kintone<'a> {
    pub fn new(base_url: &'a str, api_token: &'a str) -> Kintone<'a> {
        Kintone {
            base_url,
            api_token
        }
    }
    pub fn get_base_url(&self) -> &str {
        self.base_url
    }
    #[tokio::main]
    pub async fn get_record(self, app: i32, record_id: i32) -> Result<Value, Box<dyn std::error::Error>> {
        // TODO: create a http client in the constructor
        let mut headers = HashMap::new();
        headers.insert(String::from("X-Cybozu-API-Token"), String::from(self.api_token));
        let http = HttpClient::new(&headers);
        Ok(http.get(format!("{}k/v1/record.json?app={}&id={}", self.base_url, app, record_id)).await?)
    }

}