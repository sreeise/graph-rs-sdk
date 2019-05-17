use crate::drive::drive_item::driveinfo::DriveInfo;
use crate::drive::drive_item::value::Value;
use crate::drive::ItemResult;
use graph_error::GraphError;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use transform_request::prelude::*;

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize, FromFile, ToFile)]
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
        Err(RequestError::none_err("No available file names"))
    }
}

impl TryFrom<&mut Response> for DriveItem {
    type Error = RequestError;

    fn try_from(value: &mut Response) -> Result<Self, Self::Error> {
        let status = value.status().as_u16();
        if GraphError::is_error(status) {
            return Err(RequestError::from(GraphError::try_from(status)?));
        }

        let drive_item: DriveItem = value.json()?;
        Ok(drive_item)
    }
}

impl TryFrom<String> for DriveItem {
    type Error = RequestError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let drive_item: DriveItem = serde_json::from_str(&value)?;
        Ok(drive_item)
    }
}

impl Transform<&str> for DriveItem {
    type Err = RequestError;

    fn transform(rhs: &str) -> Result<Self, Self::Err>
    where
        Self: Serialize + for<'de> Deserialize<'de>,
    {
        let drive_item: DriveItem = serde_json::from_str(rhs)?;
        Ok(drive_item)
    }
}
