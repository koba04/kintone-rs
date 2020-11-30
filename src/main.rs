extern crate clap;
use clap::{Clap, ArgSettings};

extern crate kintone_rs;
use kintone_rs::KintoneAPIClient;

#[derive(Clap)]
#[clap(name = "kintone-rs", author = "koba04", version = "0.1.0", about = "A REST API for kintone")]
struct Opts {
    #[clap(long, env = "KINTONE_BASE_URL")]
    base_url: String,
    #[clap(long, env = "KINTONE_API_TOKEN", setting = ArgSettings::HideEnvValues)]
    api_token: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    GetRecord(GetRecord)
}

#[derive(Clap)]
struct GetRecord {
    #[clap(long)]
    app: i32,
    #[clap(long)]
    query: Option<String>,
    #[clap(multiple = true, long)]
    fields: Option<Vec<String>>,
    #[clap(long)]
    total_count: bool
}

fn main() {
    let opts: Opts = Opts::parse();

    let base_url = opts.base_url;
    let api_token = opts.api_token;
    let api_client = KintoneAPIClient::new(&base_url, &api_token);

    match opts.subcmd {
        SubCommand::GetRecord(args) => {
            let app = args.app;
            let query = args.query;
            let fields = args.fields;
            let total_count = args.total_count;
            let result = api_client.record.get_records(app, query, fields, total_count).unwrap();
            println!("{:}", result);
        },
    }
}
