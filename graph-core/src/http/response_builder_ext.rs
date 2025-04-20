use url::Url;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct HttpExtUrl(pub Url);

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct HttpExtSerdeJsonValue(pub serde_json::Value);

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct HttpExtVecU8(pub Vec<u8>);

/// Extension trait for http::response::Builder objects
///
/// Allows the user to add a `Url` to the http::Response
pub trait HttpResponseBuilderExt {
    /// A builder method for the `http::response::Builder` type that allows the user to add a `Url`
    /// to the `http::Response`
    fn url(self, url: Url) -> Self;
    fn json(self, value: &serde_json::Value) -> Self;
}

impl HttpResponseBuilderExt for http::response::Builder {
    fn url(self, url: Url) -> Self {
        self.extension(HttpExtUrl(url))
    }

    fn json(self, value: &serde_json::Value) -> Self {
        if let Ok(value) = serde_json::to_vec(value) {
            return self.extension(HttpExtVecU8(value));
        }

        self
    }
}

pub trait HttpResponseExt {
    fn url(&self) -> Option<Url>;
    fn json(&self) -> Option<serde_json::Value>;
}

impl<T> HttpResponseExt for http::Response<T> {
    fn url(&self) -> Option<Url> {
        self.extensions()
            .get::<HttpExtUrl>()
            .map(|url| url.clone().0)
    }

    fn json(&self) -> Option<serde_json::Value> {
        self.extensions()
            .get::<HttpExtVecU8>()
            .and_then(|value| serde_json::from_slice(value.0.as_slice()).ok())
    }
}
