use serde_json::{Value};

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
        let resp = reqwest::Client::new()
        .get(&format!("{}k/v1/record.json?app={}&id={}", self.base_url, app, record_id))
        .header("X-Cybozu-API-Token", self.api_token)
        .send()
        .await?
        .json::<Value>()
        .await?;
        // TODO: should not clone
        Ok(resp["record"].clone())
    }

}