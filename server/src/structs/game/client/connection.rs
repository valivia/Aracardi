use axum::http::HeaderMap;
use semver::Version;
use serde::Serialize;
use woothee::parser::Parser;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConnection {
    pub version: Version,

    #[serde(skip)]
    pub remote_addr: Option<String>,
    pub user_agent: Option<String>,
    pub country_code: Option<String>,
}

impl ClientConnection {
    pub fn new(version: Version, headers: &HeaderMap) -> Self {
        Self {
            version,
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

    pub fn get_os(&self) -> Option<String> {
        let parser = Parser::new();
        let result = parser.parse(self.user_agent.as_deref()?)?;
        Some(result.os.to_string())
    }
}
