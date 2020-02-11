use std::collections::HashMap;
use crate::http::HttpClient;

pub mod record;
use record::Record;

pub struct KintoneAPIClient<'a> {
    pub record: Box<Record<'a>>
}

impl<'a> KintoneAPIClient<'a> {
    /// constructor for KintoneAPIClient
    ///
    /// # Arguments
    ///
    /// * `base_url` - A base URL of your environment. e.g. https://example.kintone.com
    /// * `api_token` - A API token for target apps, which can be comma-separated list
    ///
    /// ```
    /// use kintone_rs::KintoneAPIClient;
    /// let client = KintoneAPIClient::new("https://example.kintone.com", "some token");
    /// ```
    ///
    /// # Properties
    ///
    /// * `record` - A Record client to deal with records
    ///
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