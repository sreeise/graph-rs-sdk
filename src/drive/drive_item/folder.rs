use crate::drive::drive_item::view::View;
use std::io::Write;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
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
}
