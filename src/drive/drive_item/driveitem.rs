use crate::drive::drive_item::driveinfo::DriveInfo;
use crate::drive::drive_item::value::Value;
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
pub struct DriveItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_info: Option<DriveInfo>,
    #[serde(rename = "@odata.context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    _odata_context: Option<String>,
    #[serde(rename = "@odata.nextLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    _odata_next_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Vec<Value>>,
}

impl DriveItem {
    pub fn new(
        drive_info: Option<DriveInfo>,
        data_context: Option<String>,
        next_link: Option<String>,
        value: Option<Vec<Value>>,
    ) -> DriveItem {
        DriveItem {
            drive_info,
            _odata_context: data_context,
            _odata_next_link: next_link,
            value,
        }
    }

    pub fn drive_info(&self) -> Option<DriveInfo> {
        self.drive_info.clone()
    }

    pub fn data_context(&self) -> Option<String> {
        self._odata_context.clone()
    }

    pub fn value(&self) -> Option<Vec<Value>> {
        self.value.clone()
    }

    // TODO: This should probably return an Option.
    // There are a few uses in the test cases and examples that need to be changed.
    pub fn value_idx(&self, idx: usize) -> Value {
        if let Some(vec) = &self.value {
            vec[idx].clone()
        } else {
            Value::default()
        }
    }

    pub fn sort_by_name(&mut self) {
        if let Some(ref mut vec) = self.value {
            vec.sort_by(|a, b| a.name().cmp(&b.name()));
        }
    }

    pub fn sort_by_id(&mut self) {
        if let Some(ref mut vec) = self.value {
            vec.sort_by(|a, b| a.name().cmp(&b.id()));
        }
    }

    pub fn file_names(&mut self) -> ItemResult<Vec<Option<String>>> {
        if let Some(ref mut vec) = self.value {
            let v: Vec<Option<String>> = vec.clone().into_par_iter().map(|i| i.name()).collect();
            return Ok(v);
        }
        Err(GraphFailure::none_err("No available file names"))
    }

    pub fn find_by_name(&mut self, name: &str) -> Option<Value> {
        if let Some(vec) = self.value() {
            if let Some(value) = vec.iter().find(|s| s.name() == Some(name.into())) {
                return Some(value.clone());
            }
        }
        None
    }

    pub fn find_by_id(&mut self, id: &str) -> Option<Value> {
        if let Some(vec) = self.value() {
            if let Some(value) = vec.iter().find(|s| s.id() == Some(id.into())) {
                return Some(value.clone());
            }
        }
        None
    }
}

impl From<DriveInfo> for DriveItem {
    fn from(drive_info: DriveInfo) -> Self {
        DriveItem::new(Some(drive_info), None, None, None)
    }
}

impl From<Vec<Value>> for DriveItem {
    fn from(value_vec: Vec<Value>) -> Self {
        DriveItem::new(None, None, None, Some(value_vec))
    }
}

impl From<Value> for DriveItem {
    fn from(value: Value) -> Self {
        let mut vec: Vec<Value> = Vec::new();
        vec.push(value);
        DriveItem::new(None, None, None, Some(vec))
    }
}

impl TryFrom<&mut Response> for DriveItem {
    type Error = GraphFailure;

    fn try_from(value: &mut Response) -> Result<Self, Self::Error> {
        let status = value.status().as_u16();
        if GraphError::is_error(status) {
            return Err(GraphFailure::from(
                GraphError::try_from(status).unwrap_or_default(),
            ));
        }

        let mut drive_item: DriveItem = value.json()?;
        if drive_item.value.is_none() {
            let value: ItemResult<Value>  = value.json().map_err(GraphFailure::from);
            if value.is_ok() {
                drive_item.set_value(Some(vec![value?]));
            }
        }
        Ok(drive_item)
    }
}

impl TryFrom<String> for DriveItem {
    type Error = GraphFailure;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut drive_item: DriveItem = serde_json::from_str(&value)?;
        if drive_item.value.is_none() {
            let value: ItemResult<Value>  = serde_json::from_str(&value).map_err(GraphFailure::from);
            if value.is_ok() {
                drive_item.set_value(Some(vec![value?]));
            }
        }
        Ok(drive_item)
    }
}

impl TryFrom<&str> for DriveItem {
    type Error = GraphFailure;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut drive_item: DriveItem = serde_json::from_str(value)?;
        if drive_item.value.is_none() {
            let value: ItemResult<Value>  = serde_json::from_str(value).map_err(GraphFailure::from);
            if value.is_ok() {
                drive_item.set_value(Some(vec![value?]));
            }
        }
        Ok(drive_item)
    }
}

impl IntoIterator for DriveItem {
    type Item = Value;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.unwrap_or_default().into_iter()
    }
}
