use crate::drive::base::driveinfo::DriveInfo;
use crate::drive::base::driveitem::DriveItem;
use crate::drive::base::value::Value;
use crate::drive::error::DriveError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseItem<T> {
    pub item: Option<T>,
    pub error: Option<DriveError>,
}

impl<T> BaseItem<T> {
    pub fn new(item: Option<T>, error: Option<DriveError>) -> BaseItem<T> {
        BaseItem { item, error }
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
        self.item.to_owned()
    }
}

impl BaseItem<DriveItem> {
    pub fn item(&self) -> Option<DriveItem> {
        self.item.to_owned()
    }
}

impl BaseItem<Value> {
    pub fn item(&self) -> Option<Value> {
        self.item.to_owned()
    }
}
