use crate::drive::driveaction::DriveEvent;
use crate::drive::{DriveVersion, GRAPH_ENDPOINT};

/// A drive resource is the top level drive and describes where the item requested
/// originates from.
///
/// # See Also:
/// [Documentation on Drive Resources](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/concepts/addressing-driveitems?view=odsp-graph-online)
///
/// [Documentation on Drive Resources Continued](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/drive_get?view=odsp-graph-online)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum DriveResource {
    Drives,
    Groups,
    Sites,
    Users,
    Me,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DriveEventPath {
    version: DriveVersion,
    resource: DriveResource,
    drive_id: Option<String>,
    item_id: String,
    drive_event: DriveEvent,
}

impl AsRef<str> for DriveResource {
    fn as_ref(&self) -> &str {
        match self {
            DriveResource::Drives => "/drives",
            DriveResource::Groups => "/groups",
            DriveResource::Sites => "/sites",
            DriveResource::Users => "/users",
            DriveResource::Me => "/me",
        }
    }
}

impl ToString for DriveResource {
    fn to_string(&self) -> String {
        match self {
            DriveResource::Drives => self.as_ref().into(),
            DriveResource::Groups => self.as_ref().into(),
            DriveResource::Sites => self.as_ref().into(),
            DriveResource::Users => self.as_ref().into(),
            DriveResource::Me => self.as_ref().into(),
        }
    }
}

impl DriveResource {
    pub fn item_resource(self, item_id: &str, drive_action: DriveEvent) -> String {
        match self {
            DriveResource::Drives => format!(
                "{}/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                item_id,
                drive_action.as_str()
            ),
            DriveResource::Groups => format!(
                "{}/groups/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                item_id,
                drive_action.as_str()
            ),
            DriveResource::Sites => format!(
                "{}/sites/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                item_id,
                drive_action.as_str()
            ),
            DriveResource::Users => format!(
                "{}/users/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                item_id,
                drive_action.as_str()
            ),
            DriveResource::Me => format!(
                "{}/me/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                item_id,
                drive_action.as_str()
            ),
        }
    }

    pub fn drive_item_resource(
        self,
        drive_id: &str,
        item_id: &str,
        drive_action: DriveEvent,
    ) -> String {
        match self {
            DriveResource::Drives => format!(
                "{}/drives/{}/items/{}/{}",
                GRAPH_ENDPOINT,
                drive_id,
                item_id,
                drive_action.as_str()
            ),
            DriveResource::Groups => format!(
                "{}/groups/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                drive_id,
                item_id,
                drive_action.as_str()
            ),
            DriveResource::Sites => format!(
                "{}/sites/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                drive_id,
                item_id,
                drive_action.as_str()
            ),
            DriveResource::Users => format!(
                "{}/users/{}/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                drive_id,
                item_id,
                drive_action.as_str()
            ),
            DriveResource::Me => format!(
                "{}/me/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                item_id,
                drive_action.as_str()
            ),
        }
    }
}

impl DriveEventPath {
    pub fn new(
        version: DriveVersion,
        resource: DriveResource,
        drive_event: DriveEvent,
        item_id: &str,
    ) -> DriveEventPath {
        DriveEventPath {
            version,
            resource,
            drive_id: None,
            item_id: item_id.into(),
            drive_event,
        }
    }

    pub fn drive_id(&mut self, id: &str) {
        self.drive_id = Some(id.into());
    }

    pub fn get_drive_version(&self) -> &DriveVersion {
        &self.version
    }

    pub fn get_drive_resource(&self) -> &DriveResource {
        &self.resource
    }

    pub fn get_drive_event(&self) -> &DriveEvent {
        &self.drive_event
    }

    pub fn get_drive_id(&self) -> Option<&String> {
        self.drive_id.as_ref()
    }

    pub fn get_item_id(&self) -> &String {
        &self.item_id
    }
}
