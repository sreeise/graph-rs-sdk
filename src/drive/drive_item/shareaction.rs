use crate::drive::drive_item::identityset::IdentitySet;
use std::io::Write;

#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct ShareAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    recipients: Option<Vec<IdentitySet>>,
}
