#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DriveResource {
    Drives,
    Groups,
    Sites,
    Users,
    Me,
}

impl DriveResource {
    pub fn as_str(&self) -> String {
        match self {
            DriveResource::Drives => String::from("/drives"),
            DriveResource::Groups => String::from("/groups"),
            DriveResource::Sites => String::from("/sites"),
            DriveResource::Users => String::from("/users"),
            DriveResource::Me => String::from("/me"),
        }
    }
}
