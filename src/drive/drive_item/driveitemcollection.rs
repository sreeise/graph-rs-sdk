use crate::drive::drive_item::driveinfo::DriveInfo;
use crate::drive::drive_item::driveitem::DriveItem;
use crate::drive::ItemResult;
use from_to_file::*;
use graph_error::GraphError;
use graph_error::GraphFailure;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use reqwest::Response;
use std::convert::TryFrom;

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize, FromToFile, Setters)]
#[set = "pub set"]
pub struct DriveItemCollection {
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_info: Option<DriveInfo>,
    #[serde(rename = "@odata.context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    _odata_context: Option<String>,
    #[serde(rename = "@odata.nextLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    _odata_next_link: Option<String>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_items: Option<Vec<DriveItem>>,
}

impl DriveItemCollection {
    pub fn new(
        drive_info: Option<DriveInfo>,
        data_context: Option<String>,
        next_link: Option<String>,
        drive_items: Option<Vec<DriveItem>>,
    ) -> DriveItemCollection {
        DriveItemCollection {
            drive_info,
            _odata_context: data_context,
            _odata_next_link: next_link,
            drive_items,
        }
    }

    pub fn drive_info(&self) -> Option<DriveInfo> {
        self.drive_info.clone()
    }

    pub fn data_context(&self) -> Option<String> {
        self._odata_context.clone()
    }

    pub fn value(&self) -> Option<Vec<DriveItem>> {
        self.drive_items.clone()
    }

    // TODO: This should probably return an Option.
    // There are a few uses in the test cases and examples that need to be changed.
    pub fn index(&self, idx: usize) -> DriveItem {
        if let Some(vec) = &self.drive_items {
            vec[idx].clone()
        } else {
            DriveItem::default()
        }
    }

    pub fn sort_by_name(&mut self) {
        if let Some(ref mut vec) = self.drive_items {
            vec.sort_by(|a, b| a.name().cmp(&b.name()));
        }
    }

    pub fn sort_by_id(&mut self) {
        if let Some(ref mut vec) = self.drive_items {
            vec.sort_by(|a, b| a.name().cmp(&b.id()));
        }
    }

    pub fn file_names(&mut self) -> ItemResult<Vec<Option<String>>> {
        if let Some(ref mut vec) = self.drive_items {
            let v: Vec<Option<String>> = vec.clone().into_par_iter().map(|i| i.name()).collect();
            return Ok(v);
        }
        Err(GraphFailure::none_err("No available file names"))
    }

    pub fn find_by_name(&mut self, name: &str) -> Option<DriveItem> {
        if let Some(vec) = self.value() {
            if let Some(value) = vec.iter().find(|s| s.name() == Some(name.into())) {
                return Some(value.clone());
            }
        }
        None
    }

    pub fn find_by_id(&mut self, id: &str) -> Option<DriveItem> {
        if let Some(vec) = self.value() {
            if let Some(value) = vec.iter().find(|s| s.id() == Some(id.into())) {
                return Some(value.clone());
            }
        }
        None
    }
}

impl From<DriveInfo> for DriveItemCollection {
    fn from(drive_info: DriveInfo) -> Self {
        DriveItemCollection::new(Some(drive_info), None, None, None)
    }
}

impl From<Vec<DriveItem>> for DriveItemCollection {
    fn from(value_vec: Vec<DriveItem>) -> Self {
        DriveItemCollection::new(None, None, None, Some(value_vec))
    }
}

impl From<DriveItem> for DriveItemCollection {
    fn from(value: DriveItem) -> Self {
        DriveItemCollection::new(None, None, None, Some(vec![value]))
    }
}

impl TryFrom<&mut Response> for DriveItemCollection {
    type Error = GraphFailure;

    fn try_from(value: &mut Response) -> Result<Self, Self::Error> {
        let status = value.status().as_u16();
        if GraphError::is_error(status) {
            return Err(GraphFailure::from(
                GraphError::try_from(status).unwrap_or_default(),
            ));
        }

        let mut drive_item: DriveItemCollection = value.json()?;
        if drive_item.drive_items.is_none() {
            let v: ItemResult<DriveItem> = value.json().map_err(GraphFailure::from);
            if v.is_ok() {
                drive_item.set_drive_items(Some(vec![v?]));
            } else {
                let vec_value: ItemResult<Vec<DriveItem>> =
                    value.json().map_err(GraphFailure::from);
                if vec_value.is_ok() {
                    drive_item.set_drive_items(Some(vec_value?));
                }
            }
        }
        Ok(drive_item)
    }
}

impl TryFrom<String> for DriveItemCollection {
    type Error = GraphFailure;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut drive_item: DriveItemCollection = serde_json::from_str(&value)?;
        if drive_item.drive_items.is_none() {
            let v: ItemResult<DriveItem> = serde_json::from_str(&value).map_err(GraphFailure::from);
            if v.is_ok() {
                drive_item.set_drive_items(Some(vec![v?]));
            } else {
                let vec_value: ItemResult<Vec<DriveItem>> =
                    serde_json::from_str(&value).map_err(GraphFailure::from);
                if vec_value.is_ok() {
                    drive_item.set_drive_items(Some(vec_value?));
                }
            }
        }
        Ok(drive_item)
    }
}

impl TryFrom<&str> for DriveItemCollection {
    type Error = GraphFailure;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut drive_item: DriveItemCollection = serde_json::from_str(value)?;
        if drive_item.drive_items.is_none() {
            let v: ItemResult<DriveItem> = serde_json::from_str(&value).map_err(GraphFailure::from);
            if v.is_ok() {
                drive_item.set_drive_items(Some(vec![v?]));
            } else {
                let vec_value: ItemResult<Vec<DriveItem>> =
                    serde_json::from_str(&value).map_err(GraphFailure::from);
                if vec_value.is_ok() {
                    drive_item.set_drive_items(Some(vec_value?));
                }
            }
        }
        Ok(drive_item)
    }
}

impl IntoIterator for DriveItemCollection {
    type Item = DriveItem;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.drive_items.unwrap_or_default().into_iter()
    }
}
