use crate::drive::drive_item::itemreference::ItemReference;
use crate::drive::DriveResource;
use crate::drive::ItemResult;

// Used for copy events.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
struct ParentReferenceCopy {
    parent_reference: ItemReference,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl ParentReferenceCopy {
    pub fn new(item_ref: ItemReference, name: Option<String>) -> ParentReferenceCopy {
        ParentReferenceCopy {
            parent_reference: item_ref,
            name,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DriveItemCopy {
    parent_reference: ItemReference,
    name: Option<String>,
    drive_resource: DriveResource,
}

impl DriveItemCopy {
    pub fn new(
        parent_reference: ItemReference,
        name: Option<String>,
        drive_resource: DriveResource,
    ) -> DriveItemCopy {
        DriveItemCopy {
            parent_reference,
            name,
            drive_resource,
        }
    }

    pub fn drive_resource(&self) -> DriveResource {
        self.drive_resource
    }

    pub fn as_json(&self) -> ItemResult<String> {
        if let Some(name) = &self.name {
            let prc = ParentReferenceCopy::new(self.parent_reference.clone(), Some(name.clone()));
            let s = serde_json::to_string_pretty(&prc)?;
            Ok(s)
        } else {
            let prc = ParentReferenceCopy::new(self.parent_reference.clone(), None);
            let s = serde_json::to_string_pretty(&prc)?;
            Ok(s)
        }
    }
}
