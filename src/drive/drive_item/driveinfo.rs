use crate::drive::drive_item::createdby::CreatedBy;
use crate::drive::drive_item::lastmodifiedby::LastModifiedBy;
use crate::drive::drive_item::owner::Owner;
use crate::drive::drive_item::quota::Quota;
use graph_error::GraphError;
use graph_error::GraphFailure;
use reqwest::Response;
use std::convert::TryFrom;
use std::io::Write;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct DriveInfo {
    #[serde(rename = "@odata.context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    odata_context: Option<String>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<String>,
    #[serde(rename = "driveType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_type: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_by: Option<CreatedBy>,
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_by: Option<LastModifiedBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<Owner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota: Option<Quota>,
}

impl DriveInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        odata_context: Option<String>,
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
            odata_context,
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
