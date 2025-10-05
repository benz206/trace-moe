use trace_moe::tracemoe::{new_client_with_key, SearchQuery, SearchResponse};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("TRACE_MOE_API_KEY").ok();
    let api_key_ref = api_key.as_deref();

    let client = new_client_with_key(api_key_ref).expect("client");

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
