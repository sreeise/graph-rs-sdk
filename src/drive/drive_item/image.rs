#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
pub struct Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<i64>,
}

impl Image {
    pub fn new(height: Option<i64>, width: Option<i64>) -> Self {
        Image { height, width }
    }

    pub fn height(&self) -> Option<i64> {
        self.height
    }

    pub fn width(&self) -> Option<i64> {
        self.width
    }
}
