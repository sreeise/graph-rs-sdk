use crate::drive::drive_item::sharedby::SharedBy;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
pub struct Shared {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
    #[serde(rename = "sharedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    shared_date_time: Option<String>,
    #[serde(rename = "sharedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
