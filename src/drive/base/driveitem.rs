use crate::drive::base::value::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveItem {
    #[serde(rename = "@odata.context")]
    _odata_context: Option<String>,
    #[serde(rename = "@odata.nextLink")]
    _odata_next_link: Option<String>,
    value: Option<Vec<Value>>,
}

impl DriveItem {
    pub fn new(
        data_context: Option<String>,
        next_link: Option<String>,
        value: Option<Vec<Value>>,
    ) -> DriveItem {
        DriveItem {
            _odata_context: data_context,
            _odata_next_link: next_link,
            value,
        }
    }

    pub fn data_context(&self) -> Option<String> {
        self._odata_context.clone()
    }

    pub fn value(&self) -> Option<Vec<Value>> {
        self.value.clone()
    }

    pub fn value_idx(&self, idx: usize) -> Value {
        let value = self
            .value
            .to_owned()
            .expect("Could not get Value struct from DriveItem");
        value[idx].clone()
    }
}
