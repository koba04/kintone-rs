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
        let params = vec![
            ("app", app.to_string()),
            ("id", record_id.to_string()),
        ];
        let res = self.http_client.get(
            "record.json",
            &params
        ).await?;
        // should not clone
        Ok(res["record"].clone())
    }
    #[tokio::main]
    pub async fn get_records(&self, app: i32, fields: Option<Vec<&str>>) -> Result<Value, Box<dyn std::error::Error>> {
        let mut params = vec![
            ("app", app.to_string()),
        ];
        if let Some(fields) = fields {
            for field in fields {
                params.push(("fields", String::from(field)))
            }
        }
        let res = self.http_client.get(
            "records.json",
            &params
        ).await?;
        // should not clone
        Ok(res["records"].clone())
    }
}