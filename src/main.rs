use trace_moe::client::Client;

#[tokio::main]
async fn main() {
    let client = Client::new("https://httpbin.org/").expect("client");

    let data: serde_json::Value = client
        .get_json("get")
        .await
        .expect("http get");

    println!("{}", data);
}
