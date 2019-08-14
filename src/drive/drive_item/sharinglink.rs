use std::io::Write;
use crate::drive::drive_item::application::Application;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/sharinglink?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct SharingLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    application: Option<Application>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    link_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<String>,
    #[serde(rename = "webHtml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    web_html: Option<String>,
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<String>,
}
