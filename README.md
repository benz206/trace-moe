# trace-moe

Rust client for the [trace.moe](https://trace.moe/) anime scene search API.

## Install

```bash
cargo add trace-moe
```

## Quickstart

```rust
use trace_moe::tracemoe::{new_client_with_key, SearchQuery, SearchResponse};

#[tokio::main]
async fn main() {
    // Optional: set TRACE_MOE_API_KEY env var for higher quota
    let api_key = std::env::var("TRACE_MOE_API_KEY").ok();
    let client = new_client_with_key(api_key.as_deref()).expect("client");

    let query = SearchQuery {
        url: Some("https://images.plurk.com/32B15UXxymfSMwKGTObY5e.jpg".to_string()),
        anilist_id: None,
        cut_borders: Some(true),
        anilist_info: Some(false),
    };

    let resp: SearchResponse = client
        .tracemoe_search_by_url(&query)
        .await
        .expect("search");

    println!("{} results, error='{}'", resp.result.len(), resp.error);
}
```

## Example

Run the included example:

```bash
cargo run --example quick
# with API key
TRACE_MOE_API_KEY=your_key cargo run --example quick
```

## License

Licensed under MIT.
