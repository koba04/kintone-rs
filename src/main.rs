use kintone::api_client::Kintone;

fn main() {
    let base_url = "https://example.com";
    let api_client = Kintone::new(base_url);
    println!("base_url is {}", api_client.get_base_url());
}
