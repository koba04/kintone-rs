use serde_json::{Value};
use crate::http::HttpClient;

pub struct Record<'a> {
    http_client: Box<HttpClient<'a>>,
}

impl<'a> Record<'a> {
    pub fn new(http_client: Box<HttpClient<'a>>) -> Record<'a> {
        Record {
            http_client
        }
    }
    #[tokio::main]
    pub async fn get_record(&self, app: i32, record_id: i32) -> Result<Value, Box<dyn std::error::Error>> {
        let res = self.http_client.get(
            &self.build_url(
                "record.json",
                &format!("app={}&id={}", app, record_id)
            )
        ).await?;
        Ok(res)
    }
    fn build_url(&self, end_point: &str, query: &str) -> String {
        format!("k/v1/{}?{}", end_point, query)
    }
}