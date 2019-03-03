use crate::drive::driveaction::DriveAction;
use crate::drive::GRAPH_ENDPOINT;

/// A drive resource is the top level drive and describes where the item requested
/// originates from.
///
/// # See Also:
/// [Documentation on Drive Resources](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/concepts/addressing-driveitems?view=odsp-graph-online)
/// [Documentation on Drive Resources Continued](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/drive_get?view=odsp-graph-online)
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DriveResource {
    Drives,
    Groups,
    Sites,
    Users,
    Me,
}

impl DriveResource {
    pub fn to_string(&self) -> String {
        match self {
            DriveResource::Drives => String::from("/drives"),
            DriveResource::Groups => String::from("/groups"),
            DriveResource::Sites => String::from("/sites"),
            DriveResource::Users => String::from("/users"),
            DriveResource::Me => String::from("/me"),
        }
    }

    pub fn action_url(
        &self,
        drive_id: Option<&str>,
        item_id: &str,
        drive_action: DriveAction,
    ) -> String {
        let d_id = match drive_id {
            Some(t) => t,
            None => return format!(
                "{}/me/drive/items/{}/{}",
                GRAPH_ENDPOINT,
                item_id,
                drive_action.as_str()
            )
        };

        match self {
            DriveResource::Drives => {
               format!(
                    "{}/drives/{}/items/{}/{}",
                    GRAPH_ENDPOINT,
                    d_id,
                    item_id,
                    drive_action.as_str()
                )
            },
            DriveResource::Groups => {
                format!(
                    "{}/groups/{}/drive/items/{}/{}",
                    GRAPH_ENDPOINT,
                    d_id,
                    item_id,
                    drive_action.as_str()
                )
            }
            DriveResource::Sites => {
             format!(
                    "{}/sites/{}/drive/items/{}/{}",
                    GRAPH_ENDPOINT,
                    d_id,
                    item_id,
                    drive_action.as_str()
                )
            },
            DriveResource::Users => {
                format!(
                        "{}/users/{}/drive/items/{}/{}",
                        GRAPH_ENDPOINT,
                        d_id,
                        item_id,
                        drive_action.as_str()
                    )
            },
            DriveResource::Me => {
                format!(
                    "{}/me/drive/items/{}/{}",
                    GRAPH_ENDPOINT,
                    item_id,
                    drive_action.as_str()
                )
            },
        }
    }
}
