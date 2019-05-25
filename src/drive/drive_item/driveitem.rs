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

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize, FromToFile)]
pub struct DriveItem {
    drive_info: Option<DriveInfo>,
    #[serde(rename = "@odata.context")]
    _odata_context: Option<String>,
    #[serde(rename = "@odata.nextLink")]
    _odata_next_link: Option<String>,
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

    pub fn value_idx(&self, idx: usize) -> Value {
        let value = self.value.to_owned().unwrap();
        value[idx].clone()
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

impl TryFrom<&mut Response> for DriveItem {
    type Error = GraphFailure;

    fn try_from(value: &mut Response) -> Result<Self, Self::Error> {
        let status = value.status().as_u16();
        if GraphError::is_error(status) {
            return Err(GraphFailure::from(GraphError::try_from(status)?));
        }

        let drive_item: DriveItem = value.json()?;
        Ok(drive_item)
    }
}

impl TryFrom<String> for DriveItem {
    type Error = GraphFailure;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let drive_item: DriveItem = serde_json::from_str(&value)?;
        Ok(drive_item)
    }
}

impl TryFrom<&str> for DriveItem {
    type Error = GraphFailure;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let drive_item: DriveItem = serde_json::from_str(value)?;
        Ok(drive_item)
    }
}
