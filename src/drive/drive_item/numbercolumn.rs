// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/numbercolumn?view=odsp-graph-online
#[derive(Default, Debug, Clone, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct NumberColumn {
    #[serde(rename = "decimalPlaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    decimal_places: Option<String>,
    #[serde(rename = "displayAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    display_as: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    maximum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    minimum: Option<f64>,
}

impl Eq for NumberColumn {}

impl PartialEq for NumberColumn {
    fn eq(&self, other: &NumberColumn) -> bool {
        self.decimal_places == other.decimal_places &&
            self.display_as == other.display_as &&
            self.maximum == other.maximum &&
            self.minimum == other.minimum
    }
}
