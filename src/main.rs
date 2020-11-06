extern crate clap;
use clap::{Clap, ArgSettings};

extern crate kintone_rs;
use kintone_rs::KintoneAPIClient;

#[derive(Clap, Debug)]
#[clap(name = "env")]
struct Opts {
    #[clap(long, env = "KINTONE_BASE_URL")]
    base_url: String,
    #[clap(long, env = "KINTONE_API_TOKEN", setting = ArgSettings::HideEnvValues)]
    api_token: Option<String>,
    #[clap(long)]
    app: i32,
    /*
    #[clap(long, env = "KINTONE_USER_NAME")]
    user_name: Option<String>,
    #[clap(long, env = "KINTONE_PASSWORD")]
    password: Option<String>,
    */
}

fn main() {
    /*
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
        .arg(
            Arg::with_name("query")
            .long("query")
            .help("set a query string you want to get")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("fields")
            .long("fields")
            .help("set field codes you want to get as comma-separated list")
            .takes_value(true)
        )
        .get_matches()
    ;
    */
    let opts: Opts = Opts::parse();

    // println!("{:?}", opts);

    let base_url = &opts.base_url;
    let api_token = &opts.api_token.unwrap();
    let app = opts.app;

    let api_client = KintoneAPIClient::new(base_url, &api_token);
    let result = api_client.record.get_records(app, None, None).unwrap();
    println!("{:}", result);

    /*
    let result: serde_json::value::Value;
    if let Some(record) = matches.value_of("record") {
        result = api_client.record.get_record(app, record.parse::<i32>().expect("app should be a number")).unwrap();
    } else {
        let mut fields = None;
        if let Some(fields_str) = matches.value_of("fields") {
            fields = Some(fields_str.split(',').collect());
        }
        let query = matches.value_of("query");
        result = api_client.record.get_records(app, query, fields).unwrap();
    }
    */
}
