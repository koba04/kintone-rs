# kintone-rs

A kintone API client for Rust language.

**THIS IS NOT AN OFFICIAL LIBRARY, JUST A HOBBY PROJECT**

## CLI

```
kintone-rs --base_url https://example.cybozu.com/ --api_token YOUR_API_TOKEN --app YOUR_APP_ID --record YOUR_RECORD_ID
```

## Crate

```rust
use kintone_rs::KintoneAPIClient;

let YOUR_APP_ID = 1;
let YOUR_RECORD_ID = 2;

let api_client = KintoneAPIClient::new("https://example.cybozu.com", "YOUR_API_TOKEN");

// Get a record
let record = api_client.record.get_record(YOUR_APP_ID, YOUR_RECORD_ID).expect("An error occured");

// Get records
let records = api_client.record.get_records(
    YOUR_APP_ID,
    Some("Company_Name = \"foo\""),
    Some(vec!["Company_Name", "Address"])
);
```

## Support APIs

- [Get Record](https://developer.kintone.io/hc/en-us/articles/213149287)

## LICENSE

MIT