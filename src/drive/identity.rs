/// Represents a modifier for a drive item or drive. May be changed in the future.
#[derive(Debug)]
pub enum Identity {
    Application,
    User,
    Group,
    Device,
}

impl Identity {
    pub fn as_str(&self) -> &str {
        match self {
            Identity::Application => "application",
            Identity::User => "user",
            Identity::Group => "group",
            Identity::Device => "device",
        }
    }
}

#[derive(Debug)]
pub struct IdentitySet {
    modified_by: Identity,
    display_name: String,
    id: String,
}
