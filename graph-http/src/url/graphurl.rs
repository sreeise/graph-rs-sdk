use graph_error::GraphFailure;
use std::ffi::OsStr;
use std::ops::{Deref, Index, Range, RangeFrom, RangeFull, RangeTo};
use std::str::FromStr;
use url::form_urlencoded::Serializer;
use url::{PathSegmentsMut, Position, Url, UrlQuery};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
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

    #[allow(clippy::result_unit_err)]
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

    pub fn to_reqwest_url(&self) -> Url {
        Url::parse(self.as_str()).unwrap()
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

    /// Retrieves the total count of matching resources.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#count-parameter)
    pub fn count(&mut self, value: &str) {
        self.append_query_pair("$count", value);
    }

    /// Filters properties (columns).
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#select-parameter)
    pub fn select(&mut self, value: &[&str]) {
        let s = value.join(",");
        self.append_query_pair("$select", &s);
    }

    /// Retrieves related resources.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#expand-parameter)
    pub fn expand(&mut self, value: &[&str]) {
        let s = value.join(" ");
        self.append_query_pair("$expand", &s);
    }

    /// Filters results (rows).
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#filter-parameter)
    pub fn filter(&mut self, value: &[&str]) {
        let s = value.join(",");
        self.append_query_pair("$filter", &s);
    }

    /// Orders results.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#orderby-parameter)
    pub fn order_by(&mut self, value: &[&str]) {
        let s = value.join(" ");
        self.append_query_pair("$orderby", &s);
    }

    /// Returns results based on search criteria.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#search-parameter)
    pub fn search(&mut self, value: &str) {
        self.append_query_pair("$search", value);
    }

    /// Returns the results in the specified media format.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#format-parameter)
    pub fn format(&mut self, value: &str) {
        self.append_query_pair("$format", value);
    }

    /// Indexes into a result set. Also used by some APIs to implement paging and can be used
    /// together with $top to manually page results.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#skip-parameter)
    pub fn skip(&mut self, value: &str) {
        self.append_query_pair("$skip", value.as_ref());
    }

    /// Retrieves the next page of results from result sets that span multiple pages.
    /// (Some APIs use $skip instead.)
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#skiptoken-parameter)
    pub fn skip_token(&mut self, value: &str) {
        self.append_query_pair("$skipToken", value.as_ref());
    }

    /// Sets the page size of results.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#top-parameter)
    pub fn top(&mut self, value: &str) {
        self.append_query_pair("$top", value.as_ref());
    }

    pub fn cast(&mut self, value: &str) {
        self.extend_path(&[value]);
    }
}

impl From<Url> for GraphUrl {
    fn from(url: Url) -> Self {
        GraphUrl { url }
    }
}

impl From<&Url> for GraphUrl {
    fn from(url: &Url) -> Self {
        GraphUrl { url: url.clone() }
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

impl Default for GraphUrl {
    fn default() -> Self {
        GraphUrl::parse("https://graph.microsoft.com/v1.0").unwrap()
    }
}
