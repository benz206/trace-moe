use reqwest::header::{HeaderName, HeaderValue};
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Result;

const DEFAULT_BASE: &str = "https://api.trace.moe/";

pub fn new_client_with_key(api_key: Option<&str>) -> Result<Client> {
    let mut client = Client::new(DEFAULT_BASE)?;
    if let Some(key) = api_key {
        client = client.with_default_header(
            HeaderName::from_static("x-trace-key"),
            HeaderValue::from_str(key).expect("valid api key header"),
        );
    }
    Ok(client)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse<TAnilist = i64> {
    pub frame_count: i64,
    pub error: String,
    pub result: Vec<SearchResult<TAnilist>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult<TAnilist = i64> {
    pub anilist: TAnilist,
    pub filename: String,
    pub episode: Option<Episode>,
    pub duration: f64,
    pub from: f64,
    pub to: f64,
    pub at: f64,
    pub similarity: f64,
    #[serde(alias = "picture")]
    pub image: String,
    pub video: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Episode {
    Number(i64),
    Text(String),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct anilist_infoTitle {
    pub native: Option<String>,
    pub romaji: Option<String>,
    pub english: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct anilist_info {
    pub id: i64,
    pub id_mal: Option<i64>,
    pub title: anilist_infoTitle,
    pub synonyms: Vec<String>,
    pub is_adult: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeResponse {
    pub id: String,
    pub priority: i64,
    #[serde(alias = "competition")]
    pub concurrency: i64,
    pub quota: i64,
    pub quota_used: i64,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct SearchQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anilist_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cut_borders: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anilist_info: Option<bool>,
}

impl Client {
    pub async fn tracemoe_search_by_url<TAnilist: for<'de> serde::Deserialize<'de>>(
        &self,
        query: &SearchQuery,
    ) -> Result<SearchResponse<TAnilist>> {
        let path = build_query_path("search", query);
        self.get_json::<SearchResponse<TAnilist>>(path).await
    }

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
        Ok(Self::parse_json(resp).await?)
    }

    pub async fn tracemoe_me(&self) -> Result<MeResponse> {
        self.get_json("me").await
    }
}

pub fn build_query_path(base: &str, query: &SearchQuery) -> String {
    let mut url = url::Url::parse("https://dummy.invalid/").unwrap();
    url.set_path(base);
    let mut qp = url::form_urlencoded::Serializer::new(String::new());
    if let Some(v) = &query.url { qp.append_pair("url", v); }
    if let Some(v) = query.anilist_id { qp.append_pair("anilist_id", &v.to_string()); }
    if let Some(true) = query.cut_borders { qp.append_pair("cut_borders", ""); }
    if let Some(true) = query.anilist_info { qp.append_pair("anilist_info", ""); }
    let qs = qp.finish();
    if qs.is_empty() { base.to_string() } else { format!("{}?{}", base, qs) }
}


