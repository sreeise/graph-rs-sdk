use crate::drive::base::driveitem::DriveItem;

use crate::drive::endpoint::DriveEndPoint;
use crate::drive::item::Item;
use crate::drive::{Drive, ItemResult};
use std::string::ToString;

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
#[derive(Display, Debug)]
pub enum QueryType<'u, 'a, 's> {
    #[strum(serialize = "?$select=")]
    Select(&'u str, &'a [&'s str]),
    #[strum(serialize = "&expand=")]
    Expand(&'u str, &'a [&'s str]),
    #[strum(serialize = "&filter=")]
    Filter(&'u str, &'a [&'s str]),
    OrderBy,
    Search,
    Format,
    Skip,
    Top,
}

impl<'u, 'a, 's> QueryType<'u, 'a, 's> {
    pub fn to_query_string(&self) -> String {
        match self {
            QueryType::Select(s, vec) => {
                let query_str = vec.join(",").clone();
                odata_query!(s.to_string(), self.to_string(), query_str.to_string())
            },
            _ => unimplemented!(),
        }
    }
}

/// Query string trait for building graph-error requests with select and expand query strings
///
/// # See Also:
/// [Query Documentation](https://docs.microsoft.com/en-us/graph/query-parameters)
/// [Query Documentation Continued](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/concepts/optional-query-parameters?view=odsp-graph-online#selecting-properties)
pub trait QueryString: Item {
    fn select(&mut self, end_point: DriveEndPoint, query: &[&str]) -> ItemResult<DriveItem>;
    fn select_url(&self, end_point: DriveEndPoint, query: &[&str]) -> String;
    fn expand(
        &mut self,
        end_point: DriveEndPoint,
        expand_item: &str,
        query: &[&str],
    ) -> ItemResult<DriveItem>;

    fn expand_url(&self, end_point: DriveEndPoint, expand_item: &str, query: &[&str]) -> String;

    fn filter(&mut self, end_point: DriveEndPoint, query_str: &[&str]) -> ItemResult<DriveItem>;

    fn filter_url(&self, end_point: DriveEndPoint, query: &[&str]) -> String;

    fn order_by(&mut self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem>;

    fn order_by_url(&self, end_point: DriveEndPoint, query: &str) -> String;

    fn search(&mut self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem>;

    fn search_url(&self, end_point: DriveEndPoint, query_str: &str) -> String;

    fn format_url(&self, end_point: DriveEndPoint, query_str: &str) -> String;

    fn format(&mut self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem>;

    // TODO: Fix skip() and top() url formatting.
    fn skip(&self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem>;

    // TODO: Fix skip() and top() url formatting.
    fn top(&self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem>;
}

impl QueryString for Drive {
    /// Query String Select
    ///
    /// Calls the drive/graph-error api with a select Odata query such as:
    ///     "https://graph.microsoft.com/v1.0/drive/root/children?select=name,size"
    ///
    /// The query should be a &Vec<&str> that holds the query parameters the caller
    /// wants to select: &vec!["name", "size"]
    ///
    ///
    /// # Example
    /// ```rust,ignore
    ///
    /// let mut drive = new Drive("AccessToken");
    ///
    /// let base_item = drive.select(DriveEndPoint::Drive, &vec!["name", "size"]);
    /// if !base_item.error.is_some() {
    ///     println!("{:#?}", &base_item); // BaseItem<DriveItem>
    /// } else {
    ///     println!("{:#?}", &base_item.error); // DriveError
    /// }
    /// ```
    fn select(&mut self, end_point: DriveEndPoint, query: &[&str]) -> ItemResult<DriveItem> {
        let url = self.select_url(end_point, query);
        self.get(url.as_str())
    }

    /// Query String Select
    ///
    /// An expand request url includes an item to expand and the items to select:
    ///     select=name,size
    ///
    /// # Example
    /// ```rust,ignore
    ///    let mut query = Vec::new();
    ///    query.push("name");
    ///    query.push("size");
    ///
    ///   // Forms the request url: "https://graph.microsoft.com/v1.0/drive/root/children?select=name,size"
    ///   // sending the request and returning the response
    ///   let mut req = drive.select(DriveEndPoint::DriveRootChild, query)?;
    ///
    ///   println!("{:#?}", req); // -> Head of response
    ///   println!("{:#?}", req.text()); // -> Body of response
    /// ```
    fn select_url(&self, end_point: DriveEndPoint, query: &[&str]) -> String {
        let query_str = query.join(",").clone();
        odata_query!(
            DriveEndPoint::build(end_point),
            "?$select=".to_string(),
            query_str.to_string()
        )
    }

    /// Query String Expand
    ///
    /// An expand request url includes an item to expand and the items to select:
    ///     expand=children(select=id,name,folder)
    ///
    /// # Example
    /// ```rust,ignore
    /// let mut query = vec!["id", "name", "size"];
    ///
    /// // Forms the request url: "https://graph.microsoft.com/v1.0/me/drive/root?expand=children(select=id,name,size)"
    /// // sending the request and returning the response
    /// let base_item: BaseItem<Value> = drive.expand(DriveEndPoint::DriveRoot, "children", query);
    ///
    /// if !base_item.error.is_some() {
    ///     println!("{:#?}", &base_item); // BaseItem<Value>
    /// } else {
    ///     println!("{:#?}", &base_item.error); // DriveError
    /// }
    ///```
    /// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/concepts/optional-query-parameters?view=odsp-graph-online#selecting-properties
    fn expand(
        &mut self,
        end_point: DriveEndPoint,
        expand_item: &str,
        query: &[&str],
    ) -> ItemResult<DriveItem> {
        let url = self.expand_url(end_point, expand_item, query);
        self.get(url.as_str())
    }

    /// Get the URL string for a expand query string
    fn expand_url(&self, end_point: DriveEndPoint, expand_item: &str, query: &[&str]) -> String {
        let query_str = query.join(",").clone();
        odata_query!(
            DriveEndPoint::build(end_point),
            "?expand=".to_string(),
            expand_item.to_string(),
            "(select=".to_string(),
            query_str,
            String::from(")")
        )
    }

    fn filter(&mut self, end_point: DriveEndPoint, query: &[&str]) -> ItemResult<DriveItem> {
        let url = self.filter_url(end_point, query);
        self.get(url.as_str())
    }

    fn filter_url(&self, end_point: DriveEndPoint, query: &[&str]) -> String {
        let query_str = query.join(" ").clone();
        odata_query!(
            DriveEndPoint::build(end_point),
            "?$filter=".to_string(),
            query_str.to_string()
        )
    }

    fn order_by(&mut self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem> {
        let url = self.order_by_url(end_point, query_str);
        self.get(url.as_str())
    }

    fn order_by_url(&self, end_point: DriveEndPoint, query_str: &str) -> String {
        odata_query!(
            DriveEndPoint::build(end_point),
            "?$orderby=".to_string(),
            query_str.to_string()
        )
    }

    fn search(&mut self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem> {
        let url = self.search_url(end_point, query_str);
        self.get(url.as_str())
    }

    fn search_url(&self, end_point: DriveEndPoint, query_str: &str) -> String {
        odata_query!(
            DriveEndPoint::build(end_point),
            "?$search=".to_string(),
            query_str.to_string()
        )
    }

    fn format_url(&self, end_point: DriveEndPoint, query_str: &str) -> String {
        odata_query!(
            DriveEndPoint::build(end_point),
            "?$format=".to_string(),
            query_str.to_string()
        )
    }

    fn format(&mut self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem> {
        let url = self.format_url(end_point, query_str);
        self.get(url.as_str())
    }

    #[allow(unused_variables)]
    fn skip(&self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem> {
        unimplemented!();
    }

    #[allow(unused_variables)]
    fn top(&self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem> {
        unimplemented!();
    }
}
