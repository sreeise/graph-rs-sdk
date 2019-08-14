use std::io::Write;
use crate::drive::drive_item::baseitem::BaseItem;
use crate::drive::drive_item::columndefinition::ColumnDefinition;
use crate::drive::drive_item::contenttype::ContentType;
use crate::drive::drive_item::drive::Drive;
use crate::drive::drive_item::list::List;
use crate::drive::drive_item::sharepointid::SharePointIds;
use crate::drive::drive_item::sitecollection::SiteCollection;
use crate::drive::Root;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/site?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct Site {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root: Option<Root>,
    #[serde(rename = "sharepointIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sharepoint_ids: Option<SharePointIds>,
    #[serde(rename = "siteCollection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    site_collection: Option<SiteCollection>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(rename = "contentTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    content_types: Option<Vec<ContentType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drive: Option<Drive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drives: Option<Vec<Drive>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<BaseItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lists: Option<Vec<List>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sites: Option<Vec<Site>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<Vec<ColumnDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "eTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    e_tag: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_date_time: Option<String>,
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<String>,
}
