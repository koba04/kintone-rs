use serde_json::{Value};
use std::collections::HashMap;
use crate::http::HttpClient;

pub struct Kintone<'a> {
    http_client: Box<HttpClient<'a>>,
    base_url: &'a str,
}

impl<'a> Kintone<'a> {
    pub fn new(base_url: &'a str, api_token: &'a str) -> Kintone<'a> {
        let mut headers = HashMap::new();
        headers.insert("X-Cybozu-API-Token", api_token);
        let http_client = Box::new(HttpClient::new(Box::new(headers)));
        Kintone {
            base_url,
            http_client
        }
    }
    pub fn get_base_url(&self) -> &str {
        self.base_url
    }
    #[tokio::main]
    pub async fn get_record(&self, app: i32, record_id: i32) -> Result<Value, Box<dyn std::error::Error>> {
        let res = self.http_client.get(
            self.build_url(
                "record.json",
                &format!("app={}&id={}", app, record_id)
            )
        ).await?;
        Ok(res)
    }
    fn build_url(&self, end_point: &str, query: &str) -> String {
        format!("{}k/v1/{}?{}", self.base_url, end_point, query)
    }
}