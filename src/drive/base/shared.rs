use crate::drive::base::sharedby::SharedBy;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Shared {
    scope: Option<String>,
    #[serde(rename = "sharedDateTime")]
    shared_date_time: Option<String>,
    #[serde(rename = "sharedBy")]
    shared_by: Option<SharedBy>,
}

impl Shared {
    pub fn new(
        scope: Option<String>,
        shared_date_time: Option<String>,
        shared_by: Option<SharedBy>,
    ) -> Self {
        Shared {
            scope,
            shared_date_time,
            shared_by,
        }
    }
}

impl Shared {
    pub fn scope(&self) -> Option<String> {
        self.scope.clone()
    }

    pub fn shared_date_time(&self) -> Option<String> {
        self.shared_date_time.clone()
    }

    pub fn shared_by(&self) -> Option<SharedBy> {
        self.shared_by.clone()
    }
}
