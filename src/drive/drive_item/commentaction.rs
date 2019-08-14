use crate::drive::drive_item::identityset::IdentitySet;
use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/commentaction?view=odsp-graph-online
#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct CommentAction {
    #[serde(rename = "isReply")]
    #[serde(skip_serializing_if = "Option::is_none")]
    is_reply: Option<bool>,
    #[serde(rename = "parentAuthor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_author: Option<IdentitySet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    participants: Option<Vec<IdentitySet>>,
}
