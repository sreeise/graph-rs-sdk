#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ParentReference {
    #[serde(rename = "driveId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_id: Option<String>,
    #[serde(rename = "driveType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
}

impl ParentReference {
    pub fn new(
        drive_id: Option<String>,
        drive_type: Option<String>,
        id: Option<String>,
        path: Option<String>,
    ) -> ParentReference {
        ParentReference {
            drive_id,
            drive_type,
            id,
            path,
        }
    }

    pub fn drive_id(&self) -> Option<String> {
        self.drive_id.clone()
    }

    pub fn drive_type(&self) -> Option<String> {
        self.drive_type.clone()
    }

    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn path(&self) -> Option<String> {
        self.path.clone()
    }
}
