use crate::drive::drive_item::itemreference::ItemReference;
use crate::drive::ItemResult;
use graph_error::GraphFailure;

// Used for copy events.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ItemRefCopy {
    parent_reference: ItemReference,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl ItemRefCopy {
    pub fn new(item_ref: ItemReference, name: Option<String>) -> ItemRefCopy {
        ItemRefCopy {
            parent_reference: item_ref,
            name,
        }
    }

    pub fn as_json(&self) -> ItemResult<String> {
        serde_json::to_string_pretty(&self).map_err(GraphFailure::from)
    }
}
