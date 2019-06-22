use crate::drive::drive_item::fieldvalueset::FieldValueSet;
use crate::drive::drive_item::identityset::IdentitySet;
use crate::drive::drive_item::publicationfacet::PublicationFacet;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/listitemversion?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct ListItemVersion {
    fields: Option<FieldValueSet>,
    id: Option<String>,
    #[serde(rename = "lastModifiedBy")]
    last_modified_by: Option<IdentitySet>,
    #[serde(rename = "lastModifiedDateTime")]
    last_modified_date_time: Option<String>,
    published: Option<PublicationFacet>,
}
