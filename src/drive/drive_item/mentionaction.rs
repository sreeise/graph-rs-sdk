use crate::drive::drive_item::identityset::IdentitySet;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/mentionaction?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct MentionAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    mentionees: Option<Vec<IdentitySet>>,
}
