use crate::drive::drive_item::commentaction::CommentAction;
use crate::drive::drive_item::deleteaction::DeleteAction;
use crate::drive::drive_item::mentionaction::MentionAction;
use crate::drive::drive_item::moveaction::MoveAction;
use crate::drive::drive_item::renameaction::RenameAction;
use crate::drive::drive_item::shareaction::ShareAction;
use crate::drive::drive_item::versionaction::VersionAction;
use std::collections::BTreeMap;
use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/itemactionset?view=odsp-graph-online
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct ItemActionSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<CommentAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    create: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delete: Option<DeleteAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edit: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mention: Option<MentionAction>,
    #[serde(rename = "move")]
    #[serde(skip_serializing_if = "Option::is_none")]
    move_action: Option<MoveAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rename: Option<RenameAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    restore: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    share: Option<ShareAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<VersionAction>,
}

impl Eq for ItemActionSet {}
