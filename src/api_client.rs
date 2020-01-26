use std::collections::HashMap;
use crate::http::HttpClient;

pub mod record;
use record::Record;

pub struct KintoneAPIClient<'a> {
    pub record: Box<Record<'a>>
}

impl<'a> KintoneAPIClient<'a> {
    pub fn new(base_url: &'a str, api_token: &'a str) -> KintoneAPIClient<'a> {

        let mut headers = HashMap::new();
        headers.insert("X-Cybozu-API-Token", api_token);
        let http_client = Box::new(HttpClient::new(Box::new(headers), base_url));
        let record = Box::new(Record::new(http_client));
        KintoneAPIClient {
            record
        }
    }
}