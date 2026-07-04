use reqwest::header::{HeaderName, HeaderValue};
use reqwest::multipart::{Form, Part};
use serde::Deserialize;

use crate::client::Client;
use crate::error::Result;

const DEFAULT_BASE: &str = "https://api.trace.moe/";

/// Create a `Client` configured for the trace.moe API.
///
/// If an API key is provided, it will be sent via the `x-trace-key` header.
/// For higher quotas, obtain a key from the trace.moe dashboard.
pub fn new_client_with_key(api_key: Option<&str>) -> Result<Client> {
    let mut client = Client::new(DEFAULT_BASE)?;
    if let Some(key) = api_key {
        client = client.with_default_header(
            HeaderName::from_static("x-trace-key"),
            HeaderValue::from_str(key)?,
        );
    }
    Ok(client)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Response returned by trace.moe search endpoints.
pub struct SearchResponse<TAnilist = i64> {
    pub frame_count: i64,
    pub error: String,
    pub result: Vec<SearchResult<TAnilist>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A single search hit.
pub struct SearchResult<TAnilist = i64> {
    pub anilist: TAnilist,
    pub filename: String,
    pub episode: Option<Episode>,
    pub duration: f64,
    pub from: f64,
    pub to: f64,
    pub at: f64,
    pub similarity: f64,
    pub image: String,
    pub video: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
/// Episode info in results may be a number or text.
pub enum Episode {
    Number(i64),
    Text(String),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// AniList title variants.
pub struct AnilistInfoTitle {
    pub native: Option<String>,
    pub romaji: Option<String>,
    pub english: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// AniList metadata when `anilist_info` is requested.
pub struct AnilistInfo {
    pub id: i64,
    pub id_mal: Option<i64>,
    pub title: AnilistInfoTitle,
    pub synonyms: Vec<String>,
    pub is_adult: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Response from `GET /me` describing quota and concurrency.
pub struct MeResponse {
    pub id: String,
    pub priority: i64,
    pub concurrency: i64,
    pub quota: i64,
    pub quota_used: i64,
}

#[derive(Default, Debug, Clone)]
/// Search query parameters for `search` endpoints.
pub struct SearchQuery {
    pub url: Option<String>,
    pub anilist_id: Option<i64>,
    pub cut_borders: Option<bool>,
    pub anilist_info: Option<bool>,
}

impl Client {
    /// Search by image URL with optional parameters.
    pub async fn tracemoe_search_by_url<TAnilist: for<'de> serde::Deserialize<'de>>(
        &self,
        query: &SearchQuery,
    ) -> Result<SearchResponse<TAnilist>> {
        let path = build_query_path("search", query);
        self.get_json::<SearchResponse<TAnilist>>(path).await
    }

    /// Upload image bytes to search.
    pub async fn tracemoe_search_upload<TAnilist: for<'de> serde::Deserialize<'de>>(
        &self,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<SearchResponse<TAnilist>> {
        let form = Form::new().part("image", Part::bytes(bytes.into()));
        let resp = self
            .request(reqwest::Method::POST, "search")?
            .multipart(form)
            .send()
            .await?;
        Self::parse_json(resp).await
    }

    /// Get current account quota and concurrency information.
    pub async fn tracemoe_me(&self) -> Result<MeResponse> {
        self.get_json("me").await
    }
}

/// Build a relative path with query string from a `SearchQuery`.
fn build_query_path(base: &str, query: &SearchQuery) -> String {
    let mut qp = url::form_urlencoded::Serializer::new(String::new());
    if let Some(v) = &query.url { qp.append_pair("url", v); }
    if let Some(v) = query.anilist_id { qp.append_pair("anilistID", &v.to_string()); }
    if let Some(true) = query.cut_borders { qp.append_pair("cutBorders", ""); }
    if let Some(true) = query.anilist_info { qp.append_pair("anilistInfo", ""); }
    let qs = qp.finish();
    if qs.is_empty() { base.to_string() } else { format!("{}?{}", base, qs) }
}
