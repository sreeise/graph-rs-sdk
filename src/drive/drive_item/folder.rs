use crate::drive::drive_item::view::View;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
pub struct Folder {
    #[serde(rename = "childCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    child_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    view: Option<View>,
}

impl Folder {
    pub fn new(child_count: Option<i64>, view: Option<View>) -> Self {
        Folder { child_count, view }
    }

    pub fn child_count(&self) -> Option<i64> {
        self.child_count
    }

    pub fn view(&self) -> Option<View> {
        self.view.clone()
    }
}
