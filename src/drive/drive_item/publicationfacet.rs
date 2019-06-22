// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/publicationfacet?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct PublicationFacet {
    #[serde(skip_serializing_if = "Option::is_none")]
    level: Option<String>,
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    version_id: Option<String>,
}
