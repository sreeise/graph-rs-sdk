use crate::graph_rs_types::entitytypes::BaseItem;
use crate::graph_rs_types::entitytypes::DriveItem;
use from_as::*;
use graph_error::GraphResult;
use graph_error::{GraphError, GraphFailure};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reqwest::Response;

/// A Collection stores a collection of items returned from the API such
/// as a collection of DriveItem's or User's.
#[derive(
    Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters, Getters, FromFile, AsFile,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct Collection<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_info: Option<BaseItem>,
    #[serde(rename = "@odata.nextLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    odata_next_link: Option<String>,
    #[serde(rename = "@odata.deltaLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    odata_delta_link: Option<String>,
    #[serde(rename = "@odata.context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    odata_context: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Vec<T>>,
}

impl<T> Collection<T> {
    pub fn new(
        drive_info: Option<BaseItem>,
        next_link: Option<String>,
        delta_link: Option<String>,
        context: Option<String>,
        value: Option<Vec<T>>,
    ) -> Collection<T> {
        Collection {
            drive_info,
            odata_next_link: next_link,
            odata_delta_link: delta_link,
            odata_context: context,
            value,
        }
    }

    pub fn index(&self, idx: usize) -> Option<&T> {
        if let Some(vec) = self.value.as_ref() {
            vec.get(idx)
        } else {
            None
        }
    }

    pub fn add(&mut self, value: T) {
        if let Some(ref mut vec) = self.value {
            vec.push(value);
        } else {
            let mut vec: Vec<T> = Vec::new();
            vec.push(value);
            self.value = Some(vec);
        }
    }

    pub fn len(&self) -> usize {
        if let Some(v) = self.value.as_ref() {
            v.len()
        } else {
            0
        }
    }

    pub fn is_empty(&self) -> bool {
        if let Some(v) = self.value.as_ref() {
            v.is_empty()
        } else {
            true
        }
    }
}

impl<T> Eq for Collection<T> where T: std::cmp::PartialEq {}

impl Collection<DriveItem> {
    pub fn sort_by_name(&mut self) {
        if let Some(ref mut vec) = self.value {
            vec.sort_by(|a, b| a.name.cmp(&b.name));
        }
    }

    pub fn sort_by_id(&mut self) {
        if let Some(ref mut vec) = self.value {
            vec.sort_by(|a, b| a.id.cmp(&b.id));
        }
    }

    pub fn file_names(&mut self) -> GraphResult<Vec<String>> {
        if let Some(ref mut vec) = self.value {
            let v: Vec<String> = vec
                .clone()
                .into_par_iter()
                .map(|i| i.name.clone())
                .flatten()
                .collect();
            return Ok(v);
        }
        Err(GraphFailure::none_err("No available file names"))
    }

    pub fn find_by_name(&mut self, name: &str) -> Option<DriveItem> {
        if let Some(vec) = self.value.as_ref() {
            if let Some(value) = vec.iter().find(|s| s.name == Some(name.into())) {
                return Some(value.clone());
            }
        }
        None
    }

    pub fn find_by_id(&mut self, id: &str) -> Option<DriveItem> {
        if let Some(vec) = self.value.as_ref() {
            if let Some(value) = vec.iter().find(|s| s.id == Some(id.into())) {
                return Some(value.clone());
            }
        }
        None
    }
}

impl<T> From<BaseItem> for Collection<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    fn from(drive_info: BaseItem) -> Self {
        Collection::new(Some(drive_info), None, None, None, None)
    }
}

impl From<Vec<DriveItem>> for Collection<DriveItem> {
    fn from(vec: Vec<DriveItem>) -> Self {
        Collection::new(None, None, None, None, Some(vec))
    }
}

impl From<DriveItem> for Collection<DriveItem> {
    fn from(drive_item: DriveItem) -> Self {
        Collection::new(None, None, None, None, Some(vec![drive_item]))
    }
}

impl<T> IntoIterator for Collection<T>
where
    T: Clone,
{
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.clone().unwrap_or_default().into_iter()
    }
}

impl TryFrom<&str> for Collection<DriveItem> {
    type Error = GraphFailure;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut c: Collection<DriveItem> = serde_json::from_str(value)?;
        if c.value.is_none() {
            let v: GraphResult<DriveItem> =
                serde_json::from_str(&value).map_err(GraphFailure::from);
            if v.is_ok() {
                c.set_value(Some(vec![v?]));
            } else {
                let vec_value: GraphResult<Vec<DriveItem>> =
                    serde_json::from_str(&value).map_err(GraphFailure::from);
                if vec_value.is_ok() {
                    c.set_value(Some(vec_value?));
                }
            }
        }
        Ok(c)
    }
}

impl TryFrom<String> for Collection<DriveItem> {
    type Error = GraphFailure;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Collection::try_from(value.as_str())
    }
}

impl TryFrom<&mut Response> for Collection<DriveItem> {
    type Error = GraphFailure;

    fn try_from(value: &mut Response) -> Result<Self, Self::Error> {
        let status = value.status().as_u16();
        if GraphError::is_error(status) {
            return Err(GraphFailure::from(
                GraphError::try_from(status).unwrap_or_default(),
            ));
        }

        let mut c: Collection<DriveItem> = value.json()?;
        if c.value.is_none() {
            let v: GraphResult<DriveItem> = value.json().map_err(GraphFailure::from);
            if v.is_ok() {
                c.set_value(Some(vec![v?]));
            } else {
                let vec_value: GraphResult<Vec<DriveItem>> =
                    value.json().map_err(GraphFailure::from);
                if vec_value.is_ok() {
                    c.set_value(Some(vec_value?));
                }
            }
        }
        Ok(c)
    }
}

impl TryFrom<&mut Response> for Collection<BaseItem> {
    type Error = GraphFailure;

    fn try_from(value: &mut Response) -> Result<Self, Self::Error> {
        let status = value.status().as_u16();
        if GraphError::is_error(status) {
            return Err(GraphFailure::from(
                GraphError::try_from(status).unwrap_or_default(),
            ));
        }

        let mut c: Collection<BaseItem> = value.json()?;
        if c.value.is_none() {
            let v: GraphResult<BaseItem> = value.json().map_err(GraphFailure::from);
            if v.is_ok() {
                c.set_value(Some(vec![v?]));
            } else {
                let vec_value: GraphResult<Vec<BaseItem>> =
                    value.json().map_err(GraphFailure::from);
                if vec_value.is_ok() {
                    c.set_value(Some(vec_value?));
                }
            }
        }
        Ok(c)
    }
}
