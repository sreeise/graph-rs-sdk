use url::Url;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct HttpExtUrl(pub Url);

/// Extension trait for http::response::Builder objects
///
/// Allows the user to add a `Url` to the http::Response
pub trait HttpResponseBuilderExt {
    /// A builder method for the `http::response::Builder` type that allows the user to add a `Url`
    /// to the `http::Response`
    fn url(self, url: Url) -> Self;
}

impl HttpResponseBuilderExt for http::response::Builder {
    fn url(self, url: Url) -> Self {
        self.extension(HttpExtUrl(url))
    }
}

pub trait HttpResponseExt {
    fn url(&self) -> Option<Url>;
}

impl<T> HttpResponseExt for http::Response<T> {
    fn url(&self) -> Option<Url> {
        self.extensions()
            .get::<HttpExtUrl>()
            .map(|url| url.clone().0)
    }
}
