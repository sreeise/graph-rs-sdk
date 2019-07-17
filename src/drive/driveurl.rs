use crate::drive::event::DriveEvent;
use crate::drive::item::Item;
use crate::drive::{Drive, DriveEndPoint, DriveResource, DriveVersion};
use crate::prelude::ResourceBuilder;
use std::convert::TryFrom;
use std::iter::Iterator;
use std::ops::{Deref, Index, Range, RangeFrom, RangeFull, RangeTo};
use url::{Position, Url};

#[derive(Debug, Clone, PartialEq)]
pub struct DriveUrl {
    url: Url,
}

impl DriveUrl {
    pub fn new(drive_version: DriveVersion) -> DriveUrl {
        DriveUrl {
            url: Url::parse(drive_version.as_ref()).unwrap(),
        }
    }

    pub fn set_host(&mut self, drive_version: DriveVersion) {
        self.url.set_host(Some(drive_version.as_ref())).unwrap();
    }

    pub fn set_path(&mut self, path: &str) -> &mut Self {
        self.url.set_path(path);
        self
    }

    pub fn join_path(&mut self, path: &str) -> &mut Self {
        if let Ok(mut p) = self.url.path_segments_mut() {
            p.push(path);
        }
        self
    }

    pub fn extend_path(&mut self, path: &[&str]) -> &mut Self {
        if let Ok(mut p) = self.url.path_segments_mut() {
            p.extend(path);
        }
        self
    }

    pub fn resource(&mut self, dr: DriveResource) -> &mut Self {
        self.join_path(dr.as_ref().trim_start_matches('/'));
        self
    }

    pub fn endpoint(&mut self, de: DriveEndPoint) -> &mut Self {
        let mut vec: Vec<&str> = de.as_str().split('/').collect();
        vec.retain(|s| !s.eq(&""));
        self.extend_path(&vec)
    }

    pub fn event(&mut self, event: DriveEvent) -> &mut Self {
        self.join_path(event.as_str());
        self
    }

    pub fn set_query(&mut self, query: &str) -> &mut Self {
        self.url.set_query(Some(query));
        self
    }

    pub fn append_query_pair(&mut self, key: &str, value: &str) -> &mut Self {
        self.url.query_pairs_mut().append_pair(key, value);
        self
    }

    pub fn as_str(&self) -> &str {
        &self.url[..]
    }
}

impl From<&Drive> for DriveUrl {
    fn from(drive: &Drive) -> Self {
        DriveUrl::new(drive.drive_version())
    }
}

impl From<DriveVersion> for DriveUrl {
    fn from(drive_version: DriveVersion) -> Self {
        DriveUrl::new(drive_version)
    }
}

impl TryFrom<Url> for DriveUrl {
    type Error = ();

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        let s: &str = &url[..];
        if !s.starts_with(DriveVersion::V1.as_ref()) && !s.starts_with(DriveVersion::V2.as_ref()) {
            return Err(());
        }
        Ok(DriveUrl { url })
    }
}

impl TryFrom<ResourceBuilder> for DriveUrl {
    type Error = ();

    fn try_from(value: ResourceBuilder) -> Result<Self, Self::Error> {
        Ok(DriveUrl {
            url: Url::parse(value.build().map_err(|_| ())?.as_str()).unwrap(),
        })
    }
}

impl Index<RangeFull> for DriveUrl {
    type Output = str;

    fn index(&self, _: RangeFull) -> &Self::Output {
        &self.url[..]
    }
}

impl Index<RangeFrom<Position>> for DriveUrl {
    type Output = str;

    fn index(&self, index: RangeFrom<Position>) -> &Self::Output {
        &self.url[index]
    }
}

impl Index<RangeTo<Position>> for DriveUrl {
    type Output = str;

    fn index(&self, index: RangeTo<Position>) -> &Self::Output {
        &self.url[index]
    }
}

impl Index<Range<Position>> for DriveUrl {
    type Output = str;

    fn index(&self, index: Range<Position>) -> &Self::Output {
        &self.url[index]
    }
}

impl AsRef<str> for DriveUrl {
    fn as_ref(&self) -> &str {
        self.deref()
    }
}

impl Deref for DriveUrl {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.url.as_str()
    }
}

impl serde::Serialize for DriveUrl {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        url_serde::serialize(&self.url, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for DriveUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        url_serde::deserialize(deserializer).map(|url: Url| DriveUrl::try_from(url).unwrap())
    }
}
