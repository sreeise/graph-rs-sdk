#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
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

    pub fn view_type(&self) -> Option<String> {
        self.view_type.clone()
    }

    pub fn sort_by(&self) -> Option<String> {
        self.sort_by.clone()
    }

    pub fn sort_order(&self) -> Option<String> {
        self.sort_order.clone()
    }
}
