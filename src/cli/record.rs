extern crate clap;
use clap::{Clap};
use serde_json::{Value};

use crate::api_client::KintoneAPIClient;

#[derive(Clap)]
pub struct GetRecord {
    #[clap(long)]
    pub app: i32,
    #[clap(long)]
    pub id: i32
}

pub fn get_record(api_client: KintoneAPIClient, args: GetRecord) -> Value {
    let app = args.app;
    let id = args.id;
    api_client.record.get_record(app, id).unwrap()
}

#[derive(Clap)]
pub struct GetRecords {
    #[clap(long)]
    pub app: i32,
    #[clap(long)]
    pub query: Option<String>,
    #[clap(multiple = true, long)]
    pub fields: Option<Vec<String>>,
    #[clap(long)]
    pub total_count: bool
}

pub fn get_records(api_client: KintoneAPIClient, args: GetRecords) -> Value {
    let app = args.app;
    let query = args.query;
    let fields = args.fields;
    let total_count = args.total_count;
    api_client.record.get_records(app, query, fields, total_count).unwrap()
}