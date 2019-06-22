// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/driverecipient?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct DriveRecipient {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    object_id: Option<String>,
}
