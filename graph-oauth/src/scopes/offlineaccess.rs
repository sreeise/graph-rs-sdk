use crate::scopes::scope::Scope;

pub enum OfflineAccessScope {
    OfflineAccess,
    WlOfflineAccess,
}

impl AsRef<str> for OfflineAccessScope {
    fn as_ref(&self) -> &str {
        match self {
            OfflineAccessScope::OfflineAccess => "offline_access",
            OfflineAccessScope::WlOfflineAccess => "wl.offline_access",
        }
    }
}

impl Scope for OfflineAccessScope {}

impl ToString for OfflineAccessScope {
    fn to_string(&self) -> String {
        self.as_str().to_string()
    }
}
