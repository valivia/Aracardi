use axum::http::HeaderMap;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConnection {
    #[serde(skip)]
    pub remote_addr: Option<String>,
    pub user_agent: Option<String>,
    pub country_code: Option<String>,
}

impl ClientConnection {
    pub fn new(headers: &HeaderMap) -> Self {
        Self {
            remote_addr: Self::parse_header(headers, "CF-Connecting-IP"),
            user_agent: Self::parse_header(headers, "User-Agent"),
            country_code: Self::parse_header(headers, "CF-IPCountry"),
        }
    }

    fn parse_header(headers: &HeaderMap, key: &str) -> Option<String> {
        headers
            .get(key)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned)
    }
}
