#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    height: Option<i64>,
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
