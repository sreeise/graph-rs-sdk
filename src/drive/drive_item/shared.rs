use crate::drive::drive_item::identityset::IdentitySet;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct Shared {
    owner: Option<IdentitySet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
    #[serde(rename = "sharedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_date_time: Option<String>,
    #[serde(rename = "sharedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_by: Option<IdentitySet>,
}
