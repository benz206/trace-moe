//! Rust client for the trace.moe anime scene search API.
//!
//! - Search by image URL or upload bytes
//! - Optional AniList info in results
//! - Check quota/limits via `me`
//!
//! Quickstart:
//!
//! ```no_run
//! use trace_moe::tracemoe::{new_client_with_key, SearchQuery, SearchResponse};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = new_client_with_key(None)?;
//!
//!     let query = SearchQuery { url: Some("https://example.com/image.jpg".into()), anilist_id: None, cut_borders: Some(true), anilist_info: Some(false) };
//!     let resp: SearchResponse = client.tracemoe_search_by_url(&query).await?;
//!     println!("{} results", resp.result.len());
//!     Ok(())
//! }
//! ```
pub mod client;
pub mod error;
pub mod tracemoe;
