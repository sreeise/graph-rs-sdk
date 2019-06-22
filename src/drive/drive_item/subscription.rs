// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/subscription?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct Subscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "clientState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    client_state: Option<String>,
    #[serde(rename = "expirationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration_date_time: Option<String>,
    #[serde(rename = "notificationUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    notification_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<String>,
    #[serde(rename = "changeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    change_type: Option<String>,
}
