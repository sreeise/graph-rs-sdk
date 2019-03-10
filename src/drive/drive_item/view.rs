#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct View {
    #[serde(rename = "viewType")]
    view_type: Option<String>,
    #[serde(rename = "sortBy")]
    sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
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
