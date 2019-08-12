use crate::drive::drive_item::driveinfo::DriveInfo;
use crate::drive::drive_item::driveitem::DriveItem;
use crate::drive::ItemResult;
use from_to_file::FromToFile;
use graph_error::{GraphError, GraphFailure};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use reqwest::Response;
use serde::export::TryFrom;
use std::io::Write;
use std::path::Path;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct Collection<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_info: Option<DriveInfo>,
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
        drive_info: Option<DriveInfo>,
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
}

impl<T> Eq for Collection<T> where T: std::cmp::PartialEq {}

impl Collection<DriveItem> {
    pub fn sort_by_name(&mut self) {
        if let Some(ref mut vec) = self.value {
            vec.sort_by(|a, b| a.name().cmp(&b.name()));
        }
    }

    pub fn sort_by_id(&mut self) {
        if let Some(ref mut vec) = self.value {
            vec.sort_by(|a, b| a.name().cmp(&b.id()));
        }
    }

    pub fn file_names(&mut self) -> ItemResult<Vec<String>> {
        if let Some(ref mut vec) = self.value {
            let v: Vec<String> = vec
                .clone()
                .into_par_iter()
                .map(|i| i.name())
                .flatten()
                .collect();
            return Ok(v);
        }
        Err(GraphFailure::none_err("No available file names"))
    }

    pub fn find_by_name(&mut self, name: &str) -> Option<DriveItem> {
        if let Some(vec) = self.value.as_ref() {
            if let Some(value) = vec.iter().find(|s| s.name() == Some(name.into())) {
                return Some(value.clone());
            }
        }
        None
    }

    pub fn find_by_id(&mut self, id: &str) -> Option<DriveItem> {
        if let Some(vec) = self.value.as_ref() {
            if let Some(value) = vec.iter().find(|s| s.id() == Some(id.into())) {
                return Some(value.clone());
            }
        }
        None
    }
}

impl<T> From<DriveInfo> for Collection<T> {
    fn from(drive_info: DriveInfo) -> Self {
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
            let v: ItemResult<DriveItem> = serde_json::from_str(&value).map_err(GraphFailure::from);
            if v.is_ok() {
                c.set_value(Some(vec![v?]));
            } else {
                let vec_value: ItemResult<Vec<DriveItem>> =
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
            let v: ItemResult<DriveItem> = value.json().map_err(GraphFailure::from);
            if v.is_ok() {
                c.set_value(Some(vec![v?]));
            } else {
                let vec_value: ItemResult<Vec<DriveItem>> =
                    value.json().map_err(GraphFailure::from);
                if vec_value.is_ok() {
                    c.set_value(Some(vec_value?));
                }
            }
        }
        Ok(c)
    }
}

impl<T> FromToFile for Collection<T>
where
    T: serde::Serialize,
    for<'de> T: serde::Deserialize<'de>,
{
    type Err = GraphFailure;
    type Output = ();

    fn to_json_file<P: AsRef<Path>>(&self, path: P) -> Result<Self::Output, Self::Err> {
        if path.as_ref().exists() {
            std::fs::remove_file(&path)?;
        }

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)?;
        let serialized = serde_json::to_string(&self)?;
        file.write_all(serialized.as_bytes())?;
        file.sync_all()?;
        Ok(())
    }

    fn from_json_file<P: AsRef<Path>>(path: P) -> Result<Self, Self::Err> {
        let f = std::fs::File::open(path)?;
        let self_as_json: Collection<T> = serde_json::from_reader(f)?;
        Ok(self_as_json)
    }

    fn to_yaml_file<P: AsRef<Path>>(&self, path: P) -> Result<Self::Output, Self::Err> {
        if path.as_ref().exists() {
            std::fs::remove_file(&path)?;
        }

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)?;
        let serialized = serde_yaml::to_string(&self)?;
        file.write_all(serialized.as_bytes())?;
        file.sync_all()?;
        Ok(())
    }

    fn from_yaml_file<P: AsRef<Path>>(path: P) -> Result<Self, Self::Err> {
        let f = std::fs::File::open(path)?;
        let self_as_json: Collection<T> = serde_yaml::from_reader(f)?;
        Ok(self_as_json)
    }
}
