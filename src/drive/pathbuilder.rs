use crate::drive::driveresource::ResourceBuilder;
use crate::drive::item::Item;
use crate::drive::{Drive, DriveEndPoint, DriveResource, DriveVersion};
use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelExtend,
    ParallelIterator,
};
use std::collections::btree_map::BTreeMap;
use std::convert::TryFrom;
use std::iter::Iterator;
use transform_request::prelude::*;
use url::form_urlencoded::Serializer;
use url::Url;

/// A URL/URI builder for the OneDrive API.
///
/// Includes various conversion for use with rust-onedrive.
#[derive(
    Debug, Eq, PartialEq, Serialize, Deserialize, FromFile, ToFile, FromYamlFile, ToYamlFile,
)]
pub struct PathBuilder {
    scheme: String,
    host: String,
    path: Vec<String>,
    query_map: BTreeMap<String, String>,
}

impl PathBuilder {
    pub fn new(host: &str) -> PathBuilder {
        let url = Url::parse(host).unwrap();
        let query: BTreeMap<String, String> = url.query_pairs().into_owned().collect();

        let vec_path: Vec<&str> = url.path().split('/').collect();
        let path: Vec<String> = vec_path
            .into_par_iter()
            .map(std::string::ToString::to_string)
            .filter(|s| !s.is_empty())
            .collect();

        let mut host = url.host().unwrap().to_string();
        if host.ends_with('/') {
            host.truncate(host.len() - 1);
        }

        PathBuilder {
            scheme: url.scheme().into(),
            host,
            path,
            query_map: query,
        }
    }

    pub fn path<T>(&mut self, path: T) -> &mut Self
    where
        T: ToString,
    {
        let mut p = path.to_string();
        if p.starts_with('/') {
            p.remove(0);
        }

        if p.ends_with('/') {
            p.remove(p.len() - 1);
        }
        self.path.push(p);
        self
    }

    pub fn query<T>(&mut self, key: T, value: T) -> &mut Self
    where
        T: ToString,
    {
        self.query_map.insert(key.to_string(), value.to_string());
        self
    }

    pub fn drive_endpoint(&mut self, endpoint: DriveEndPoint) -> &mut Self {
        self.path(endpoint.as_str());
        self
    }

    pub fn drive_resource(&mut self, resource: DriveResource) -> &mut Self {
        self.path(resource.as_ref());
        self
    }

    pub fn build(&mut self) -> String {
        let mut url_vec = vec![self.scheme.as_str(), "://", self.host.as_str()];

        if !self.path.is_empty() {
            self.path
                .par_iter_mut()
                .filter(|s| !s.starts_with('/'))
                .for_each(|s| s.insert_str(0, "/"));
            let p: Vec<&str> = self.path.par_iter().map(|s| &**s).collect();
            url_vec.par_extend(p.into_par_iter());
        }

        let mut url = url_vec.join("");
        if !self.query_map.is_empty() {
            if url.ends_with('/') {
                url.truncate(url.len() - 1);
            }

            url.push_str("?&");
            let mut serializer = Serializer::new(String::new());
            serializer.extend_pairs(self.query_map.iter());
            let query = serializer.finish();
            url.push_str(query.as_str());
            url
        } else {
            if url.ends_with('/') {
                url.truncate(url.len() - 1);
            }
            url
        }
    }
}

impl From<&str> for PathBuilder {
    fn from(s: &str) -> Self {
        PathBuilder::new(s)
    }
}

impl From<String> for PathBuilder {
    fn from(s: String) -> Self {
        PathBuilder::new(s.as_str())
    }
}

// Uses the v1.0 endpoint for OneDrive.
impl From<DriveEndPoint> for PathBuilder {
    fn from(endpoint: DriveEndPoint) -> Self {
        PathBuilder::from(endpoint.v1_url())
    }
}

impl From<DriveVersion> for PathBuilder {
    fn from(version: DriveVersion) -> Self {
        PathBuilder::from(version.to_string())
    }
}

impl From<&Drive> for PathBuilder {
    fn from(drive: &Drive) -> Self {
        PathBuilder::from(drive.drive_version())
    }
}

impl TryFrom<ResourceBuilder> for PathBuilder {
    type Error = RequestError;

    fn try_from(value: ResourceBuilder) -> Result<Self, Self::Error> {
        Ok(PathBuilder::from(value.build()?))
    }
}
