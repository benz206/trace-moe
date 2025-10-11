# trace-moe

Rust client for the [trace.moe](https://trace.moe/) anime scene search API.

## Features

- Simple async client built on `reqwest`
- Search by image URL or upload bytes
- Optional AniList info in results
- Query your quota/limits via `me`

## Install

```bash
cargo add trace-moe
```

## Quickstart

```rust
use trace_moe::tracemoe::{new_client_with_key, SearchQuery, SearchResponse, Episode};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("TRACE_MOE_API_KEY").ok();
    let client = new_client_with_key(api_key.as_deref()).expect("client");

    let query = SearchQuery {
        url: Some("https://i.imgur.com/DLHIbog.png".to_string()),
        anilist_id: None,
        cut_borders: Some(true),
        anilist_info: Some(false),
    };

    let resp: SearchResponse = client
        .tracemoe_search_by_url(&query)
        .await
        .expect("search");

    println!("{} results, error='{}'", resp.result.len(), resp.error);
    for (idx, r) in resp.result.iter().enumerate() {
        let episode = match &r.episode {
            Some(Episode::Number(n)) => n.to_string(),
            Some(Episode::Text(t)) => t.clone(),
            None => "-".to_string(),
        };
        println!(
            "#{}  sim={:.2}%  anilist={}  file={}  ep={}  time={:.2}-{:.2}\n  image={}\n  video={}\n",
            idx + 1,
            r.similarity * 100.0,
            r.anilist,
            r.filename,
            episode,
            r.from,
            r.to,
            r.image,
            r.video,
        );
    }
}
```

## Usage

### Authentication

- API key is optional for low-volume usage. For higher quotas, set `TRACE_MOE_API_KEY` and use `new_client_with_key(Some(key))`.
- The client sends the key as `x-trace-key`.

### Search by URL

```rust
let query = SearchQuery {
    url: Some("https://example.com/image.jpg".into()),
    anilist_id: None,
    cut_borders: Some(true),
    anilist_info: Some(true),
};
let results: SearchResponse = client.tracemoe_search_by_url(&query).await?;
```

### Upload bytes

```rust
let bytes = std::fs::read("./frame.jpg")?;
let results: SearchResponse = client.tracemoe_search_upload(bytes).await?;
```

### Check quota/limits

```rust
let me = client.tracemoe_me().await?;
println!("quota={}, used={}", me.quota, me.quota_used);
```

## API Overview

- `new_client_with_key(Option<&str>) -> Client`
- `Client::tracemoe_search_by_url(&SearchQuery) -> SearchResponse<TAnilist>`
- `Client::tracemoe_search_upload(bytes) -> SearchResponse<TAnilist>`
- `Client::tracemoe_me() -> MeResponse`
- Types: `SearchQuery`, `SearchResponse<TAnilist>`, `SearchResult<TAnilist>`, `Episode`, `AnilistInfo`, `AnilistInfoTitle`, `MeResponse`

Docs: https://docs.rs/trace-moe

## Examples

Run the included example:

```bash
cargo run --example quick
# with API key
TRACE_MOE_API_KEY=your_key cargo run --example quick
```

## License

Licensed under MIT.
