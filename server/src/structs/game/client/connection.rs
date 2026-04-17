use axum::http::HeaderMap;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ClientConnection {
    pub remote_addr: Option<String>,
    pub user_agent: Option<String>,
    pub country_code: Option<String>,
}

impl ClientConnection {
    pub fn new(headers: &HeaderMap) -> Self {
        Self {
            remote_addr: headers
                .get("CF-Connecting-IP")
                .and_then(|value| value.to_str().ok())
                .map(str::to_owned),
            user_agent: headers
                .get("User-Agent")
                .and_then(|value| value.to_str().ok())
                .map(str::to_owned),
            country_code: headers
                .get("CF-IPCountry")
                .and_then(|value| value.to_str().ok())
                .map(str::to_owned),
        }
    }
}
