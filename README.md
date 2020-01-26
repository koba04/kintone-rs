# rust-kintone

A kintone API client for Rust language.

**THIS IS NOT AN OFFICIAL LIBRARY, JUST A HOBBY PROJECT**

## CLI

```
rust-kintone --base_url https://example.cybozu.com/ --api_token YOUR_API_TOKEN --app YOUR_APP_ID --record YOUR_RECORD_ID
```

## Crate

```rust
use kintone::api_client::Kintone;

let YOUR_APP_ID = 1;
let YOUR_RECORD_ID = 2;

let api_client = Kintone::new("https://example.cybozu.com", "YOUR_API_TOKEN");
let result = api_client.get_record(YOUR_APP_ID, YOUR_RECORD_ID).expect("An error occured");
```

## Support APIs

- [Get Record](https://developer.kintone.io/hc/en-us/articles/213149287)

## LICENSE

MIT