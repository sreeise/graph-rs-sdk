use graph_error::GraphFailure;
use std::ffi::OsStr;
use std::iter::Iterator;
use std::ops::{Deref, Index, Range, RangeFrom, RangeFull, RangeTo};
use std::str::FromStr;
use url::form_urlencoded::Serializer;
use url::{PathSegmentsMut, Position, Url, UrlQuery};

#[derive(Debug, Clone, PartialEq)]
pub struct GraphUrl {
    url: Url,
}

impl GraphUrl {
    pub fn parse(input: &str) -> Result<Self, GraphFailure> {
        Ok(GraphUrl {
            url: Url::parse(input)?,
        })
    }

    pub fn as_str(&self) -> &str {
        self.url.as_str()
    }

    pub fn set_host(&mut self, host: &str) {
        self.url.set_host(Some(host)).unwrap_or_default();
    }

    pub fn host(&self) -> Option<&str> {
        self.url.host_str()
    }

    pub fn path(&self) -> &str {
        self.url.path()
    }

    pub fn set_path<I: AsRef<str>>(&mut self, path: I) {
        self.url.set_path(path.as_ref());
    }

    pub fn extend_path<I: AsRef<str>>(&mut self, path: &[I]) {
        if let Ok(mut p) = self.url.path_segments_mut() {
            p.extend(path);
        }
    }

    pub fn query(&self) -> Option<&str> {
        self.url.query()
    }

    pub fn set_query<I: AsRef<str>>(&mut self, query: I) {
        self.url.set_query(Some(query.as_ref()));
    }

    pub fn append_query_pair<KV: AsRef<str>>(&mut self, key: KV, value: KV) {
        self.url
            .query_pairs_mut()
            .append_pair(key.as_ref(), value.as_ref());
    }

    pub fn path_segments_mutable(&mut self) -> Result<PathSegmentsMut, ()> {
        self.url.path_segments_mut()
    }

    pub fn extend_path_os_str_lossy(&mut self, path: &[&OsStr]) -> &mut Self {
        if let Ok(mut p) = self.url.path_segments_mut() {
            p.extend(path.iter().map(|s| s.to_string_lossy()));
        }
        self
    }

    pub fn to_url(&self) -> Url {
        self.url.clone()
    }

    pub fn query_pairs_mutable(&mut self) -> Serializer<UrlQuery> {
        self.url.query_pairs_mut()
    }

    pub fn starts_with(&self, start: &str) -> bool {
        self.as_str().starts_with(start)
    }

    pub fn ends_with(&self, end: &str) -> bool {
        self.as_str().ends_with(end)
    }

    pub fn replace(&mut self, input: &str) -> Result<(), GraphFailure> {
        self.url = Url::parse(input)?;
        Ok(())
    }

    pub fn count(&mut self, value: &str) {
        self.append_query_pair("count", value);
    }

    pub fn select(&mut self, value: &[&str]) {
        let s = value.join(",");
        self.append_query_pair("select", &s);
    }

    pub fn expand(&mut self, value: &[&str]) {
        let s = value.join(" ");
        self.append_query_pair("expand", &s);
    }

    pub fn filter(&mut self, value: &[&str]) {
        let s = value.join(",");
        self.append_query_pair("filter", &s);
    }

    pub fn order_by(&mut self, value: &[&str]) {
        let s = value.join(" ");
        self.append_query_pair("orderby", &s);
    }

    pub fn search(&mut self, value: &str) {
        self.append_query_pair("search", value);
    }

    pub fn format(&mut self, value: &str) {
        self.append_query_pair("format", value);
    }

    pub fn skip(&mut self, value: &str) {
        self.append_query_pair("skip", value.as_ref());
    }

    pub fn top(&mut self, value: &str) {
        self.append_query_pair("top", value.as_ref());
    }
}

impl From<Url> for GraphUrl {
    fn from(url: Url) -> Self {
        GraphUrl { url }
    }
}

impl From<reqwest::Url> for GraphUrl {
    fn from(url: reqwest::Url) -> Self {
        GraphUrl::parse(url.as_str()).unwrap()
    }
}

impl FromStr for GraphUrl {
    type Err = GraphFailure;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        GraphUrl::parse(s)
    }
}

impl Index<RangeFull> for GraphUrl {
    type Output = str;

    fn index(&self, _: RangeFull) -> &Self::Output {
        &self.url[..]
    }
}

impl Index<RangeFrom<Position>> for GraphUrl {
    type Output = str;

    fn index(&self, index: RangeFrom<Position>) -> &Self::Output {
        &self.url[index]
    }
}

impl Index<RangeTo<Position>> for GraphUrl {
    type Output = str;

    fn index(&self, index: RangeTo<Position>) -> &Self::Output {
        &self.url[index]
    }
}

impl Index<Range<Position>> for GraphUrl {
    type Output = str;

    fn index(&self, index: Range<Position>) -> &Self::Output {
        &self.url[index]
    }
}

impl AsRef<str> for GraphUrl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<Url> for GraphUrl {
    fn as_ref(&self) -> &Url {
        &self.url
    }
}

impl AsMut<Url> for GraphUrl {
    fn as_mut(&mut self) -> &mut Url {
        &mut self.url
    }
}

impl ToString for GraphUrl {
    fn to_string(&self) -> String {
        self.url[..].to_string()
    }
}

impl Deref for GraphUrl {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.url.as_str()
    }
}

impl serde::Serialize for GraphUrl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        url_serde::serialize(&self.url, serializer)
    }
}

#[allow(clippy::redundant_closure)]
impl<'de> serde::Deserialize<'de> for GraphUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        url_serde::deserialize(deserializer).map(|url: Url| GraphUrl::from(url))
    }
}
