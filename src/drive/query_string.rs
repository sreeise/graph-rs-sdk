use crate::drive::base::driveitem::DriveItem;
use crate::drive::base::value::Value;
use crate::drive::baseitem::BaseItem;
use crate::drive::endpoint::DriveEndPoint;
use crate::drive::DriveResponse;

#[macro_export]
macro_rules! odata_query {
    (
        $($query_item:expr),*
    ) => {
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}", $query_item));
            )*
            v.join("")
        }
    };
}

// TODO: Change QueryString to use enum.
#[allow(dead_code)]
enum QueryType {
    Select,
    Expand,
    Filter,
    OrderBy,
    Search,
    Format,
    Skip,
    Top,
}

/// Query string trait for building graph requests with select and expand query strings
///
/// # See Also:
/// [Query Documentation](https://docs.microsoft.com/en-us/graph/query-parameters)
/// [Query Documentation Continued](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/concepts/optional-query-parameters?view=odsp-graph-online#selecting-properties)
pub trait QueryString {
    fn select(&mut self, end_point: DriveEndPoint, query: &[&str]) -> BaseItem<DriveItem>;

    fn select_url(&self, end_point: DriveEndPoint, query: &[&str]) -> String;

    fn expand(
        &mut self,
        end_point: DriveEndPoint,
        expand_item: &str,
        query: &[&str],
    ) -> BaseItem<Value>;

    fn expand_url(&self, end_point: DriveEndPoint, expand_item: &str, query: &[&str]) -> String;

    fn filter(&mut self, end_point: DriveEndPoint, query_str: &[&str]) -> BaseItem<DriveItem>;

    fn filter_url(&self, end_point: DriveEndPoint, query: &[&str]) -> String;

    fn order_by(&mut self, end_point: DriveEndPoint, query_str: &str) -> BaseItem<DriveItem>;

    fn order_by_url(&self, end_point: DriveEndPoint, query: &str) -> String;

    fn search(&mut self, end_point: DriveEndPoint, query_str: &str) -> BaseItem<DriveItem>;

    fn search_url(&self, end_point: DriveEndPoint, query_str: &str) -> String;

    fn format_url(&self, end_point: DriveEndPoint, query_str: &str) -> String;

    fn format(&mut self, end_point: DriveEndPoint, query_str: &str) -> BaseItem<DriveItem>;

    // TODO: Fix skip() and top() url formatting.
    fn skip(&self, end_point: DriveEndPoint, query_str: &str) -> DriveResponse;

    // TODO: Fix skip() and top() url formatting.
    fn top(&self, end_point: DriveEndPoint, query_str: &str) -> DriveResponse;
}
