// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/geocoordinates?view=odsp-graph-online
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct GeoCoordinates {
    #[serde(skip_serializing_if = "Option::is_none")]
    altitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    longitude: Option<f64>,
}

impl Eq for GeoCoordinates {}
