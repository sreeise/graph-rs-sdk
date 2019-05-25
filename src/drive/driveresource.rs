use crate::drive::driveaction::DriveEvent;
use crate::drive::{DriveVersion, ItemResult};
use graph_error::GraphFailure;

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
pub struct ResourceBuilder {
    version: DriveVersion,
    resource: Option<DriveResource>,
    drive_id: Option<String>,
    item_id: Option<String>,
    drive_event: Option<DriveEvent>,
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
    pub fn item_resource(
        self,
        version: DriveVersion,
        item_id: &str,
        drive_action: DriveEvent,
    ) -> String {
        match self {
            DriveResource::Drives => format!(
                "{}/drive/items/{}/{}",
                version,
                item_id,
                drive_action.as_str()
            ),
            DriveResource::Groups => format!(
                "{}/groups/drive/items/{}/{}",
                version,
                item_id,
                drive_action.as_str()
            ),
            DriveResource::Sites => format!(
                "{}/sites/drive/items/{}/{}",
                version,
                item_id,
                drive_action.as_str()
            ),
            DriveResource::Users => format!(
                "{}/users/drive/items/{}/{}",
                version,
                item_id,
                drive_action.as_str()
            ),
            DriveResource::Me => format!(
                "{}/me/drive/items/{}/{}",
                version,
                item_id,
                drive_action.as_str()
            ),
        }
    }

    pub fn drive_item_resource(
        self,
        version: DriveVersion,
        drive_id: &str,
        item_id: &str,
        drive_action: DriveEvent,
    ) -> String {
        match self {
            DriveResource::Drives => format!(
                "{}/drives/{}/items/{}/{}",
                version,
                drive_id,
                item_id,
                drive_action.as_str()
            ),
            DriveResource::Groups => format!(
                "{}/groups/{}/drive/items/{}/{}",
                version,
                drive_id,
                item_id,
                drive_action.as_str()
            ),
            DriveResource::Sites => format!(
                "{}/sites/{}/drive/items/{}/{}",
                version,
                drive_id,
                item_id,
                drive_action.as_str()
            ),
            DriveResource::Users => format!(
                "{}/users/{}/drive/items/{}/{}",
                version,
                drive_id,
                item_id,
                drive_action.as_str()
            ),
            DriveResource::Me => format!(
                "{}/me/drive/items/{}/{}",
                version,
                item_id,
                drive_action.as_str()
            ),
        }
    }
}

impl ResourceBuilder {
    pub fn new(version: DriveVersion) -> ResourceBuilder {
        ResourceBuilder {
            version,
            resource: None,
            drive_id: None,
            item_id: None,
            drive_event: None,
        }
    }

    pub fn drive_id(&mut self, id: &str) -> &mut Self {
        self.drive_id = Some(id.into());
        self
    }

    pub fn drive_version(&mut self, version: DriveVersion) -> &mut Self {
        self.version = version;
        self
    }

    pub fn item_id(&mut self, item_id: &str) -> &mut Self {
        self.item_id = Some(item_id.into());
        self
    }

    pub fn resource(&mut self, resource: DriveResource) -> &mut Self {
        self.resource = Some(resource);
        self
    }

    pub fn drive_event(&mut self, drive_event: DriveEvent) -> &mut Self {
        self.drive_event = Some(drive_event);
        self
    }

    pub fn get_drive_version(&self) -> DriveVersion {
        self.version
    }

    pub fn get_drive_resource(&self) -> Option<&DriveResource> {
        self.resource.as_ref()
    }

    pub fn get_drive_event(&self) -> Option<&DriveEvent> {
        self.drive_event.as_ref()
    }

    pub fn get_drive_id(&self) -> Option<&String> {
        self.drive_id.as_ref()
    }

    pub fn get_item_id(&self) -> Option<&String> {
        self.item_id.as_ref()
    }

    pub fn build(&self) -> ItemResult<String> {
        if let Some(drive_id) = self.get_drive_id() {
            Ok(self
                .get_drive_resource()
                .ok_or_else(|| GraphFailure::none_err("Missing DriveResource"))?
                .drive_item_resource(
                    self.get_drive_version(),
                    drive_id,
                    self.get_item_id()
                        .ok_or_else(|| GraphFailure::none_err("Missing item id"))?,
                    self.get_drive_event()
                        .ok_or_else(|| GraphFailure::none_err("Missing DriveEvent"))?
                        .clone(),
                ))
        } else {
            Ok(self
                .get_drive_resource()
                .ok_or_else(|| GraphFailure::none_err("Missing DriveResource"))?
                .item_resource(
                    self.get_drive_version(),
                    self.get_item_id()
                        .ok_or_else(|| GraphFailure::none_err("Missing item id"))?,
                    self.get_drive_event()
                        .ok_or_else(|| GraphFailure::none_err("Missing DriveEvent"))?
                        .clone(),
                ))
        }
    }
}
