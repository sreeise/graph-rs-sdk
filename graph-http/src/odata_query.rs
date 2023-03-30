pub trait ODataQuery<RHS = Self>
where
    Self: Sized,
{
    fn append_query_pair<KV: AsRef<str> + serde::Serialize>(self, key: KV, value: KV) -> Self;

    /// Retrieves the total count of matching resources.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#count-parameter)
    fn count<S: AsRef<str>>(self, value: S) -> Self {
        self.append_query_pair("$count", value.as_ref())
    }

    /// Filters properties (columns).
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#select-parameter)
    fn select(self, value: &[&str]) -> Self {
        let s = value.join(",");
        self.append_query_pair("$select", &s)
    }

    /// Retrieves related resources.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#expand-parameter)
    fn expand(self, value: &[&str]) -> Self {
        let s = value.join(" ");
        self.append_query_pair("$expand", &s)
    }

    /// Filters results (rows).
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#filter-parameter)
    fn filter(self, value: &[&str]) -> Self {
        let s = value.join(",");
        self.append_query_pair("$filter", &s)
    }

    /// Orders results.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#orderby-parameter)
    fn order_by(self, value: &[&str]) -> Self {
        let s = value.join(" ");
        self.append_query_pair("$orderby", &s)
    }

    /// Returns results based on search criteria.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#search-parameter)
    fn search<S: AsRef<str>>(self, value: S) -> Self {
        self.append_query_pair("$search", value.as_ref())
    }

    /// Returns the results in the specified media format.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#format-parameter)
    fn format<S: AsRef<str>>(self, value: S) -> Self {
        self.append_query_pair("$format", value.as_ref())
    }

    /// Indexes into a result set. Also used by some APIs to implement paging and can be used
    /// together with $top to manually page results.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#skip-parameter)
    fn skip<S: AsRef<str>>(self, value: S) -> Self {
        self.append_query_pair("$skip", value.as_ref())
    }

    /// Retrieves the next page of results from result sets that span multiple pages.
    /// (Some APIs use $skip instead.)
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#skiptoken-parameter)
    fn skip_token<S: AsRef<str>>(self, value: S) -> Self {
        self.append_query_pair("$skipToken", value.as_ref())
    }

    /// Retrieves the next page of results from result sets that span multiple pages.
    /// (Some APIs use $skip instead.)
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#skiptoken-parameter)
    fn delta_token<S: AsRef<str>>(self, value: S) -> Self {
        self.append_query_pair("$deltaToken", value.as_ref())
    }

    /// Sets the page size of results.
    /// [See the docs](https://docs.microsoft.com/en-us/graph/query-parameters#top-parameter)
    fn top<S: AsRef<str>>(self, value: S) -> Self {
        self.append_query_pair("$top", value.as_ref())
    }
}
