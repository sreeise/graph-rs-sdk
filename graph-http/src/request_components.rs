use std::io::ErrorKind;

use graph_core::resource::ResourceIdentity;
use graph_error::{GraphFailure, GraphResult};
use http::{HeaderMap, Method};
use url::Url;

/// Provides the necessary components for building a request.
#[derive(Clone, Debug)]
pub struct RequestComponents {
    pub resource_identity: ResourceIdentity,
    pub url: Url,
    pub method: Method,
    pub headers: HeaderMap,
}

impl AsRef<Url> for RequestComponents {
    fn as_ref(&self) -> &Url {
        &self.url
    }
}

impl AsMut<Url> for RequestComponents {
    fn as_mut(&mut self) -> &mut Url {
        &mut self.url
    }
}

impl AsMut<HeaderMap> for RequestComponents {
    fn as_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }
}

impl RequestComponents {
    pub fn new(resource_identity: ResourceIdentity, url: Url, method: Method) -> RequestComponents {
        RequestComponents {
            resource_identity,
            url,
            method,
            headers: HeaderMap::with_capacity(2),
        }
    }

    pub fn query<T: serde::Serialize + ?Sized>(&mut self, query: &T) -> GraphResult<()> {
        let mut pairs = self.url.query_pairs_mut();
        let serializer = serde_urlencoded::Serializer::new(&mut pairs);

        if query.serialize(serializer).is_err() {
            return Err(GraphFailure::from(std::io::Error::new(
                ErrorKind::InvalidData,
                "Unable to serialize query",
            )));
        }

        Ok(())
    }
}

impl TryFrom<(ResourceIdentity, reqwest::Method, GraphResult<Url>)> for RequestComponents {
    type Error = GraphFailure;

    fn try_from(value: (ResourceIdentity, Method, GraphResult<Url>)) -> Result<Self, Self::Error> {
        Ok(RequestComponents::new(value.0, value.2?, value.1))
    }
}

impl Default for RequestComponents {
    fn default() -> Self {
        RequestComponents {
            resource_identity: Default::default(),
            url: Url::parse("https://graph.microsoft.com/v1.0").unwrap(),
            method: Default::default(),
            headers: Default::default(),
        }
    }
}
