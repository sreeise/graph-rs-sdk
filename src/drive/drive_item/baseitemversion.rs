use crate::drive::drive_item::lastmodifiedby::LastModifiedBy;
use crate::drive::drive_item::publicationfacet::PublicationFacet;
use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/baseitemversion?view=odsp-graph-online
#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct BaseItemVersion {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_by: Option<LastModifiedBy>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    publication: Option<PublicationFacet>,
}
