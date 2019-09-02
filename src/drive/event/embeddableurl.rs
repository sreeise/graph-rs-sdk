use from_as::*;

/// The body of a request for embeddable file previews.
/// [Embeddable file previews](https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_preview?view=odsp-graph-online)
#[derive(
    Default, Debug, Clone, PartialEq, Serialize, Deserialize, AsFile, FromFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct EmbeddableUrl {
    #[serde(skip_serializing_if = "Option::is_none")]
    viewer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chromeless: Option<bool>,
    #[serde(rename = "allowEdit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_edit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zoom: Option<f64>,
}
