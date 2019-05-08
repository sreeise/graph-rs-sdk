use crate::drive::drive_item::driveitem::DriveItem;
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

macro_rules! convert_query {
    ($drive_end_point:expr, $query:expr, $query_str:expr) => {
        odata_query!(
            $drive_end_point.to_string(),
            $query.to_string(),
            $query_str.to_string()
        )
    };
}

/// Query string trait for building graph-error requests with select and expand query strings.
///
/// For now each method only allows selecting one type of query, meaning that you would
/// have to concat another query onto the end to add more OData queries.
///
///
/// # Example
/// To return just the URL String:
/// ```
/// use rust_onedrive::drive::{Drive, DriveVersion, DriveEndPoint};
/// use rust_onedrive::drive::query_string::QueryString;
///
/// let mut drive = Drive::new("AccessToken", DriveVersion::V1);
/// let vec = vec!["name", "size"];
///
/// assert_eq!(
///     drive.select_url(DriveEndPoint::Drive, &vec),
///     "https://graph.microsoft.com/v1.0/drive?$select=name,size"
/// );
/// ```
///
///
/// # Example
/// To send a request to the API:
/// ```rust,ignore
/// let drive_item: DriveItem = drive.select(DriveEndPoint::Drive, &vec!["name", "size"]).unwrap();
/// println!("{:#?}", drive_item);
/// ```
/// # See Also:
/// [Query Documentation](https://docs.microsoft.com/en-us/graph/query-parameters)
///
/// [Query Documentation Continued](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/concepts/optional-query-parameters?view=odsp-graph-online#selecting-properties)
pub trait QueryString {
    fn count(&mut self, end_point: DriveEndPoint, query: &str) -> ItemResult<DriveItem>;

    fn count_url(&self, end_point: DriveEndPoint, query: &str) -> String;

    fn select(&mut self, end_point: DriveEndPoint, query: &[&str]) -> ItemResult<DriveItem>;

    fn select_url(&self, end_point: DriveEndPoint, query: &[&str]) -> String;

    fn expand(&mut self, end_point: DriveEndPoint, expand_item: &str) -> ItemResult<DriveItem>;

    fn expand_url(&self, end_point: DriveEndPoint, expand_item: &str) -> String;

    fn filter(&mut self, end_point: DriveEndPoint, query_str: &[&str]) -> ItemResult<DriveItem>;

    fn filter_url(&self, end_point: DriveEndPoint, query: &[&str]) -> String;

    fn order_by(&mut self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem>;

    fn order_by_url(&self, end_point: DriveEndPoint, query: &str) -> String;

    fn search(&mut self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem>;

    fn search_url(&self, end_point: DriveEndPoint, query_str: &str) -> String;

    fn format(&mut self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem>;

    fn format_url(&self, end_point: DriveEndPoint, query_str: &str) -> String;

    fn skip(&self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem>;

    fn skip_url(&self, end_point: DriveEndPoint, query_str: &str) -> String;

    fn top(&self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem>;

    fn top_url(&self, end_point: DriveEndPoint, query_str: &str) -> String;
}

impl QueryString for Drive {
    /// Query Count Request
    ///
    /// Use the $count query parameter to include a count of the total number of items in a
    /// collection alongside the page of data values returned from Microsoft Graph.
    ///
    /// # Example
    /// ```rust,ignore
    ///
    /// let mut drive = new Drive("AccessToken");
    ///
    /// let drive_item: DriveItem = drive.count(DriveEndPoint::DriveRootChild, "2").unwrap();
    /// println!("{:#?}", drive_item);
    /// ```
    fn count(&mut self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem> {
        let url = self.count_url(end_point, query_str);
        self.get(url.as_str())
    }

    /// Query Count URL
    ///
    /// # Example
    /// ```
    /// # use rust_onedrive::drive::{Drive, DriveVersion, DriveEndPoint};
    /// # use rust_onedrive::drive::query_string::QueryString;
    ///
    /// let mut drive = Drive::new("AccessToken", DriveVersion::V1);
    /// assert_eq!(
    ///     drive.count_url(DriveEndPoint::DriveRootChild, "2"),
    ///     "https://graph.microsoft.com/v1.0/drive/root/children?$count=2"
    /// );
    /// ```
    fn count_url(&self, end_point: DriveEndPoint, query_str: &str) -> String {
        convert_query!(end_point, "?$count=", query_str)
    }

    /// Query Select Request
    ///
    /// Use the $select query parameter to return a set of properties that are different than
    /// the default set for an individual resource or a collection of resources. With $select,
    /// you can specify a subset or a superset of the default properties.
    ///
    /// The query should be a &Vec<&str> that holds the query parameters the caller
    /// wants to select: &vec!["name", "size"]
    ///
    /// # Example
    /// ```rust,ignore
    ///
    /// let mut drive = new Drive("AccessToken");
    ///
    /// let drive_item: DriveItem = drive.select(DriveEndPoint::Drive, &vec!["name", "size"]).unwrap();
    /// println!("{:#?}", drive_item);
    /// ```
    fn select(&mut self, end_point: DriveEndPoint, query_str: &[&str]) -> ItemResult<DriveItem> {
        let url = self.select_url(end_point, query_str);
        self.get(url.as_str())
    }

    /// Query Select URL
    ///
    /// The query should be a &Vec<&str> that holds the query parameters the caller
    /// wants to select: &vec!["name", "size"]
    ///
    /// # Example
    /// ```
    /// # use rust_onedrive::drive::{Drive, DriveVersion, DriveEndPoint};
    /// # use rust_onedrive::drive::query_string::QueryString;
    ///
    /// let mut drive = Drive::new("AccessToken", DriveVersion::V1);
    /// let vec = vec!["name", "size"];
    ///
    /// assert_eq!(
    ///     drive.select_url(DriveEndPoint::Drive, &vec),
    ///     "https://graph.microsoft.com/v1.0/drive?$select=name,size"
    /// );
    /// ```
    fn select_url(&self, end_point: DriveEndPoint, query_str: &[&str]) -> String {
        convert_query!(end_point, "?$select=", query_str.join(",").clone())
    }

    /// Query Expand Request
    ///
    /// Use the $expand query string parameter to include the expanded resource or
    /// collection referenced by a single relationship (navigation property) in your results.
    ///
    /// # Example
    /// ```rust,ignore
    ///
    /// let mut drive = new Drive("AccessToken");
    ///
    /// let drive_item: DriveItem = drive.expand(DriveEndPoint::Drive, "children").unwrap();
    /// println!("{:#?}", drive_item);
    /// ```
    fn expand(&mut self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem> {
        let url = self.expand_url(end_point, query_str);
        self.get(url.as_str())
    }

    /// Query Expand URL
    ///
    /// # Example
    /// ```
    /// # use rust_onedrive::drive::{Drive, DriveVersion, DriveEndPoint};
    /// # use rust_onedrive::drive::query_string::QueryString;
    ///
    /// let mut drive = Drive::new("AccessToken", DriveVersion::V1);
    /// assert_eq!(
    ///     drive.expand_url(DriveEndPoint::Drive, "children"),
    ///     "https://graph.microsoft.com/v1.0/drive?$expand=children"
    /// )
    /// ```
    fn expand_url(&self, end_point: DriveEndPoint, query_str: &str) -> String {
        convert_query!(end_point, "?$expand=", query_str)
    }

    /// Query Filter Request
    ///
    /// Use the $filter query parameter to retrieve just a subset of a collection.
    ///
    /// # Example
    /// ```rust,ignore
    ///
    /// let mut drive = new Drive("AccessToken");
    /// let filter_vec = vec!["Subject", "eq", "'Welcome'"];
    ///
    /// let drive_item: DriveItem = drive.filter_url(DriveEndPoint::Drive, &filter_vec).unwrap();
    /// println!("{:#?}", drive_item);
    /// ```
    fn filter(&mut self, end_point: DriveEndPoint, query_str: &[&str]) -> ItemResult<DriveItem> {
        let url = self.filter_url(end_point, query_str);
        self.get(url.as_str())
    }

    /// Query Filter URL
    ///
    /// # Example
    /// ```
    /// # use rust_onedrive::drive::{Drive, DriveVersion, DriveEndPoint};
    /// # use rust_onedrive::drive::query_string::QueryString;
    ///
    /// let mut drive = Drive::new("AccessToken", DriveVersion::V1);
    /// let filter_vec = vec!["Subject", "eq", "'Welcome'"];
    ///
    /// assert_eq!(
    ///     drive.filter_url(DriveEndPoint::Drive, &filter_vec),
    ///     "https://graph.microsoft.com/v1.0/drive?$filter=Subject eq 'Welcome'"
    /// );
    /// ```
    fn filter_url(&self, end_point: DriveEndPoint, query_str: &[&str]) -> String {
        convert_query!(end_point, "?$filter=", query_str.join(" ").clone())
    }

    /// Query Order By Request
    ///
    /// Use the $orderby query parameter to specify the sort order of the items
    /// returned from Microsoft Graph.
    ///
    /// # Example
    /// ```rust,ignore
    ///
    /// let mut drive = new Drive("AccessToken");
    ///
    /// let drive_item: DriveItem = drive.order_by(DriveEndPoint::Drive, "name"),
    /// println!("{:#?}", drive_item);
    /// ```
    fn order_by(&mut self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem> {
        let url = self.order_by_url(end_point, query_str);
        self.get(url.as_str())
    }

    /// Query Order By URL
    ///
    /// # Example
    /// ```
    /// # use rust_onedrive::drive::{Drive, DriveVersion, DriveEndPoint};
    /// # use rust_onedrive::drive::query_string::QueryString;
    ///
    /// let mut drive = Drive::new("AccessToken", DriveVersion::V1);
    ///
    /// assert_eq!(
    ///     drive.order_by_url(DriveEndPoint::Drive, "name"),
    ///    "https://graph.microsoft.com/v1.0/drive?$orderby=name"
    /// );
    /// ```
    fn order_by_url(&self, end_point: DriveEndPoint, query_str: &str) -> String {
        convert_query!(end_point, "?$orderby=", query_str)
    }

    /// Query Search Request
    ///
    /// Use the $search query parameter to restrict the results of a request to match a
    /// search criterion.
    ///
    /// # Example
    /// ```rust,ignore
    ///
    /// let mut drive = new Drive("AccessToken");
    ///
    /// let drive_item: DriveItem = drive.search(DriveEndPoint::Drive, "pizza").unwrap();
    /// println!("{:#?}", drive_item);
    /// ```
    fn search(&mut self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem> {
        let url = self.search_url(end_point, query_str);
        self.get(url.as_str())
    }

    /// Query Search URL
    ///
    /// # Example
    /// ```
    /// # use rust_onedrive::drive::{Drive, DriveVersion, DriveEndPoint};
    /// # use rust_onedrive::drive::query_string::QueryString;
    ///
    /// let mut drive = Drive::new("AccessToken", DriveVersion::V1);
    ///
    /// assert_eq!(
    ///     drive.search_url(DriveEndPoint::Drive, "pizza"),
    ///     "https://graph.microsoft.com/v1.0/drive?$search=pizza"
    /// );
    /// ```
    fn search_url(&self, end_point: DriveEndPoint, query_str: &str) -> String {
        convert_query!(end_point, "?$search=", query_str)
    }

    /// Query Format Request
    ///
    /// Use the $format query parameter to specify the media format of the items
    /// returned from Microsoft Graph.
    ///
    /// # Example
    /// ```rust,ignore
    ///
    /// let mut drive = new Drive("AccessToken");
    ///
    /// let drive_item: DriveItem = drive.format(DriveEndPoint::Drive, "json").unwrap();
    /// println!("{:#?}", drive_item);
    /// ```
    fn format(&mut self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem> {
        let url = self.format_url(end_point, query_str);
        self.get(url.as_str())
    }

    /// Query Format URL
    ///
    /// # Example
    /// ```
    /// # use rust_onedrive::drive::{Drive, DriveVersion, DriveEndPoint};
    /// # use rust_onedrive::drive::query_string::QueryString;
    ///
    /// let mut drive = Drive::new("AccessToken", DriveVersion::V1);
    ///
    /// assert_eq!(
    ///     drive.format_url(DriveEndPoint::Drive, "json"),
    ///     "https://graph.microsoft.com/v1.0/drive?$format=json"
    /// );
    /// ```
    fn format_url(&self, end_point: DriveEndPoint, query_str: &str) -> String {
        convert_query!(end_point, "?$format=", query_str)
    }

    /// Query Skip Request
    ///
    /// Use the $skip query parameter to set the number of items to skip at
    /// the start of a collection.
    ///
    /// # Example
    /// ```rust,ignore
    ///
    /// let mut drive = new Drive("AccessToken");
    ///
    /// let drive_item: DriveItem = drive.skip(DriveEndPoint::Drive, "10").unwrap();
    /// println!("{:#?}", drive_item);
    /// ```
    fn skip(&self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem> {
        let url = self.skip_url(end_point, query_str);
        self.get(url.as_str())
    }

    /// Query Skip URL
    ///
    /// # Example
    /// ```
    /// # use rust_onedrive::drive::{Drive, DriveVersion, DriveEndPoint};
    /// # use rust_onedrive::drive::query_string::QueryString;
    ///
    /// let mut drive = Drive::new("AccessToken", DriveVersion::V1);
    ///
    /// assert_eq!(
    ///     drive.skip_url(DriveEndPoint::Drive, "5"),
    ///     "https://graph.microsoft.com/v1.0/drive?$skip=5"
    /// );
    /// ```
    fn skip_url(&self, end_point: DriveEndPoint, query_str: &str) -> String {
        convert_query!(end_point, "?$skip=", query_str)
    }

    /// Query Top Request
    ///
    /// Use the $top query parameter to specify the page size of the result set.
    ///
    /// # Example
    /// ```rust,ignore
    ///
    /// let mut drive = new Drive("AccessToken");
    ///
    /// let drive_item: DriveItem = drive.top(DriveEndPoint::Drive, "2").unwrap();
    /// println!("{:#?}", drive_item);
    /// ```
    fn top(&self, end_point: DriveEndPoint, query_str: &str) -> ItemResult<DriveItem> {
        let url = self.top_url(end_point, query_str);
        self.get(url.as_str())
    }

    /// Query Search URL
    ///
    /// # Example
    /// ```
    /// # use rust_onedrive::drive::{Drive, DriveVersion, DriveEndPoint};
    /// # use rust_onedrive::drive::query_string::QueryString;
    ///
    /// let mut drive = Drive::new("AccessToken", DriveVersion::V1);
    ///
    /// assert_eq!(
    ///     drive.top_url(DriveEndPoint::Drive, "2"),
    ///     "https://graph.microsoft.com/v1.0/drive?$top=2"
    /// );
    /// ```
    fn top_url(&self, end_point: DriveEndPoint, query_str: &str) -> String {
        convert_query!(end_point, "?$top=", query_str)
    }
}
