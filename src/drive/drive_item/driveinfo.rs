use crate::drive::drive_item::createdby::CreatedBy;
use crate::drive::drive_item::lastmodifiedby::LastModifiedBy;
use crate::drive::drive_item::owner::Owner;
use crate::drive::drive_item::quota::Quota;
use from_to_file::*;
use graph_error::GraphError;
use graph_error::GraphFailure;
use reqwest::Response;
use std::convert::TryFrom;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromToFile)]
pub struct DriveInfo {
    #[serde(rename = "@odata.context")]
    _odata_context: Option<String>,
    #[serde(rename = "createdDateTime")]
    created_date_time: Option<String>,
    description: Option<String>,
    id: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    last_modified_date_time: Option<String>,
    name: Option<String>,
    #[serde(rename = "webUrl")]
    web_url: Option<String>,
    #[serde(rename = "driveType")]
    drive_type: Option<String>,
    #[serde(rename = "createdBy")]
    created_by: Option<CreatedBy>,
    #[serde(rename = "lastModifiedBy")]
    last_modified_by: Option<LastModifiedBy>,
    owner: Option<Owner>,
    quota: Option<Quota>,
}

impl DriveInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        _odata_context: Option<String>,
        created_date_time: Option<String>,
        description: Option<String>,
        id: Option<String>,
        last_modified_date_time: Option<String>,
        name: Option<String>,
        web_url: Option<String>,
        drive_type: Option<String>,
        created_by: Option<CreatedBy>,
        last_modified_by: Option<LastModifiedBy>,
        owner: Option<Owner>,
        quota: Option<Quota>,
    ) -> Self {
        DriveInfo {
            _odata_context,
            created_date_time,
            description,
            id,
            last_modified_date_time,
            name,
            web_url,
            drive_type,
            created_by,
            last_modified_by,
            owner,
            quota,
        }
    }

    pub fn data_context(&self) -> Option<String> {
        self._odata_context.clone()
    }

    pub fn created_date_time(&self) -> Option<String> {
        self.created_date_time.clone()
    }

    pub fn description(&self) -> Option<String> {
        self.description.clone()
    }

    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    pub fn last_modified_date_time(&self) -> Option<String> {
        self.last_modified_date_time.clone()
    }

    pub fn web_url(&self) -> Option<String> {
        self.web_url.clone()
    }

    pub fn created_by(&self) -> Option<CreatedBy> {
        self.created_by.clone()
    }

    pub fn drive_type(&self) -> Option<String> {
        self.drive_type.clone()
    }

    pub fn owner(&self) -> Option<Owner> {
        self.owner.clone()
    }

    pub fn quota(&self) -> Option<Quota> {
        self.quota.clone()
    }
}

impl TryFrom<&mut Response> for DriveInfo {
    type Error = GraphFailure;

    fn try_from(value: &mut Response) -> Result<Self, Self::Error> {
        let status = value.status().as_u16();
        if GraphError::is_error(status) {
            return Err(GraphFailure::from(GraphError::try_from(status)?));
        }

        let drive_item: DriveInfo = value.json()?;
        Ok(drive_item)
    }
}

impl TryFrom<String> for DriveInfo {
    type Error = GraphFailure;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let drive_item: DriveInfo = serde_json::from_str(&value)?;
        Ok(drive_item)
    }
}
