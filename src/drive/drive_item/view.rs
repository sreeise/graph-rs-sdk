use std::io::Write;

#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct View {
    #[serde(rename = "viewType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    view_type: Option<String>,
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<String>,
}

impl View {
    pub fn new(
        view_type: Option<String>,
        sort_by: Option<String>,
        sort_order: Option<String>,
    ) -> Self {
        View {
            view_type,
            sort_by,
            sort_order,
        }
    }
}
