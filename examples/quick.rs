use trace_moe::tracemoe::{new_client_with_key, SearchQuery, SearchResponse, Episode};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("TRACE_MOE_API_KEY").ok();
    let api_key_ref = api_key.as_deref();

    let client = new_client_with_key(api_key_ref).expect("client");

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
