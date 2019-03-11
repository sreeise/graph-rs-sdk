use crate::drive::drive_item::driveinfo::DriveInfo;
use crate::drive::drive_item::facet::Value;
use graph_error::GraphError;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use transform_request::{RequestError, Transform};

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
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
}

impl Transform<&mut Response> for DriveItem {
    type Err = RequestError;

    fn transform(rhs: &mut Response) -> Result<Self, Self::Err>
    where
        Self: Serialize + for<'de> Deserialize<'de>,
    {
        let status = rhs.status().as_u16();
        if GraphError::is_error(status) {
            return Err(RequestError::from(GraphError::from(status)));
        }

        let drive_item: DriveItem = rhs.json()?;
        Ok(drive_item)
    }
}

impl Transform<String> for DriveItem {
    type Err = RequestError;

    fn transform(rhs: String) -> Result<Self, Self::Err>
    where
        Self: Serialize + for<'de> Deserialize<'de>,
    {
        let drive_item: DriveItem = serde_json::from_str(&rhs)?;
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