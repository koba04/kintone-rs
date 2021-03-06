use serde_json::{Value};
use serde::{Serialize,Deserialize};
use crate::http::HttpClient;

pub struct Record<'a> {
    http_client: Box<HttpClient<'a>>,
}


#[derive(Deserialize, Clone, Serialize)]
pub struct GetRecordResponse {
    pub record: Value
}

#[derive(Deserialize, Clone, Serialize)]
pub struct GetRecordsResponse {
    pub records: Value,
    pub totalCount: Value
}

impl<'a> Record<'a> {
    ///
    /// constructor for RecordClient
    ///
    /// # Arguments
    ///
    /// * `http_client` - A HTTP Client
    ///
    /// ```
    /// use kintone_rs::KintoneAPIClient;
    /// let client = KintoneAPIClient::new("https://example.kintone.com", "some token");
    /// let record_client = client.record;
    /// ```
    pub fn new(http_client: Box<HttpClient<'a>>) -> Record<'a> {
        Record {
            http_client
        }
    }
    /// get a record from an app
    ///
    /// # Arguments
    ///
    /// * `app` - The app ID
    /// * `record_id` - The record ID you want to get
    ///
    /// ```
    /// use kintone_rs::KintoneAPIClient;
    /// let client = KintoneAPIClient::new("https://example.kintone.com", "some token");
    /// let record = client.record.get_record(1, 10);
    /// ```
    #[tokio::main]
    pub async fn get_record(&self, app: i32, record_id: i32) -> Result<GetRecordResponse, Box<dyn std::error::Error>> {
        let params = vec![
            ("app", app.to_string()),
            ("id", record_id.to_string()),
        ];
        let res = self.http_client.get::<GetRecordResponse>(
            "record.json",
            &params
        ).await?;
        // should not clone
        Ok(res.clone())
    }
    /// get records from an app
    ///
    /// # Arguments
    ///
    /// * `app` - The app ID
    /// * `query` - A query string for conditions
    /// * `fields` - fields list you want to get
    /// * `total_count` - A boolean whether returning the total count
    /// ```
    /// use kintone_rs::KintoneAPIClient;
    /// let client = KintoneAPIClient::new("https://example.kintone.com", "some token");
    /// let records = client.record.get_records(
    ///     1,
    ///     Some(String::from("Company_Name = \"foo\"")),
    ///     Some(vec![String::from("Company_Name"), String::from("Address")]),
    ///     false
    /// );
    /// ```
    #[tokio::main]
    pub async fn get_records(&self, app: i32, query: Option<String>, fields: Option<Vec<String>>, total_count: bool) -> Result<GetRecordsResponse, Box<dyn std::error::Error>> {
        let mut params = vec![
            ("app", app.to_string()),
        ];
        if let Some(fields) = fields {
            for field in fields {
                params.push(("fields", field));
            }
        }
        if let Some(query) = query {
            params.push(("query", query));
        }
        if total_count {
            params.push(("totalCount", String::from("true")))
        }
        let res = self.http_client.get::<GetRecordsResponse>(
            "records.json",
            &params
        ).await?;
        // FIXME: should not clone
        Ok(res.clone())
    }
}