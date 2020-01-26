extern crate clap;
use clap::{Arg, App};

extern crate kintone_rs;
use kintone_rs::KintoneAPIClient;

fn main() {
    let matches = App::new("rust-kintone")
        .version("0.1.0")
        .author("koba04")
        .about("API Client for kintone")
        .arg(
            Arg::with_name("base_url")
                .long("base_url")
                .help("set URL for your kintone environment like https://test.cybozu.com")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("api_token")
                .long("api_token")
                .help("set API Token you want to use")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("app")
            .long("app")
            .help("set an App ID you want to get")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("record")
            .long("record")
            .help("set an Record ID you want to get")
            .takes_value(true)
        )
        .get_matches()
    ;

    let base_url = matches.value_of("base_url").expect("base_url is required option");
    let api_token = matches.value_of("api_token").expect("api_token is required option");
    let app = matches.value_of("app").expect("app is required option").parse::<i32>().expect("app should be a number");
    let record = matches.value_of("record").expect("record is required option").parse::<i32>().expect("record should be a number");

    let api_client = KintoneAPIClient::new(base_url, api_token);

    let result = api_client.record.get_record(app, record).unwrap();
    println!("{:}", result);
}
