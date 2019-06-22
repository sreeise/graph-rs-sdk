use crate::drive::drive_item::identityset::IdentitySet;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct ShareAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    recipients: Option<Vec<IdentitySet>>,
}
