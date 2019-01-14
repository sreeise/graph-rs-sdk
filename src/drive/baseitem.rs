use crate::drive::driveitem::DriveInfo;
use crate::drive::driveitem::DriveItem;
use crate::drive::driveitem::Value;
use crate::drive::error::DriveError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseItem<T> {
    pub item: Option<T>,
    pub error: Option<DriveError>,
    pub is_error: bool,
}

impl<T> BaseItem<T> {
    pub fn new(item: Option<T>, error: Option<DriveError>, is_error: bool) -> BaseItem<T> {
        BaseItem {
            item,
            error,
            is_error,
        }
    }

    pub fn is_error(&self) -> bool {
        self.is_error
    }

    pub fn is_none(&self) -> bool {
        self.item.is_none()
    }

    pub fn is_some(&self) -> bool {
        self.item.is_some()
    }
}

impl BaseItem<DriveInfo> {
    pub fn item(&self) -> Option<DriveInfo> {
        self.item.clone()
    }
}

impl BaseItem<DriveItem> {
    pub fn item(&self) -> Option<DriveItem> {
        self.item.clone()
    }
}

impl BaseItem<Value> {
    pub fn item(&self) -> Option<Value> {
        self.item.clone()
    }
}
