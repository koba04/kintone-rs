extern crate clap;
use clap::{Clap, ArgSettings};

extern crate kintone_rs;
use kintone_rs::KintoneAPIClient;
use kintone_rs::cli::record::*;
use kintone_rs::printer;

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
    GetRecords(GetRecords),
    GetRecord(GetRecord)
}

fn main() {
    let opts: Opts = Opts::parse();

    let base_url = opts.base_url;
    let api_token = opts.api_token;
    let api_client = KintoneAPIClient::new(&base_url, &api_token);

    match opts.subcmd {
        SubCommand::GetRecord(args) => {
            let result = get_record(api_client, args);
            printer::print(result);
        }
        SubCommand::GetRecords(args) => {
            let result = get_records(api_client, args);
            printer::print(result);
        },
    }
}
