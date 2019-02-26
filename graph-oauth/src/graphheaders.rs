use reqwest::header::HeaderMap;

/// The headers in an API request.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct GraphHeaders {
    url: String,
    status: u16,
    #[serde(skip)]
    header_map: HeaderMap,
}

impl From<&mut reqwest::Response> for GraphHeaders {
    fn from(r: &mut reqwest::Response) -> Self {
        GraphHeaders {
            url: r.url().as_str().to_string(),
            status: r.status().as_u16(),
            header_map: r.headers().to_owned(),
        }
    }
}
