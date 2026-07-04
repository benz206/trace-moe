use std::time::Duration;

use reqwest::{header::HeaderMap, Client as HttpClient, Method, RequestBuilder, Response, Url};

use crate::error::{ApiError, Result};

#[derive(Clone, Debug)]
/// Low-level HTTP client wrapper for the trace.moe API.
pub struct Client {
    base_url: Url,
    http: HttpClient,
    default_headers: HeaderMap,
}

impl Client {
    /// Create a new client with the given base URL.
    pub fn new(base_url: &str) -> Result<Self> {
        let http = HttpClient::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("trace-moe-api-wrapper/0.1")
            .build()?;

        Ok(Self {
            base_url: Url::parse(base_url)?,
            http,
            default_headers: HeaderMap::new(),
        })
    }

    /// Add a header that will be included on all requests.
    pub fn with_default_header(mut self, key: reqwest::header::HeaderName, value: reqwest::header::HeaderValue) -> Self {
        self.default_headers.insert(key, value);
        self
    }

    /// Build a request against a relative `path` under `base_url`.
    pub(crate) fn request(&self, method: Method, path: &str) -> Result<RequestBuilder> {
        let url = self.base_url.join(path)?;
        Ok(self.http.request(method, url).headers(self.default_headers.clone()))
    }

    /// Execute a GET request and deserialize JSON.
    pub async fn get_json<T: serde::de::DeserializeOwned>(&self, path: impl AsRef<str>) -> Result<T> {
        let resp = self.request(Method::GET, path.as_ref())?.send().await?;
        Self::parse_json(resp).await
    }

    /// Read the response body, map non-2xx to `ApiError::Http`, then parse JSON.
    pub(crate) async fn parse_json<T: serde::de::DeserializeOwned>(resp: Response) -> Result<T> {
        let status = resp.status();
        let text = resp.text().await?;
        if !status.is_success() {
            return Err(ApiError::Http {
                status,
                body: text,
            });
        }
        let value = serde_json::from_str::<T>(&text)?;
        Ok(value)
    }
}
