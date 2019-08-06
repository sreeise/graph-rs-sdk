use crate::drive::event::DriveEvent;
use crate::drive::{DriveEndPoint, DriveVersion};
use std::convert::TryFrom;
use std::ffi::OsString;
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

    pub fn v1() -> DriveUrl {
        DriveUrl::new(DriveVersion::V1)
    }

    pub fn v2() -> DriveUrl {
        DriveUrl::new(DriveVersion::V2)
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

    pub fn extend_path<I: AsRef<str>>(&mut self, path: &[I]) -> &mut Self {
        if let Ok(mut p) = self.url.path_segments_mut() {
            p.extend(path);
        }
        self
    }

    pub fn extend_path_os_str(&mut self, path: &[OsString]) -> &mut Self {
        if let Ok(mut p) = self.url.path_segments_mut() {
            p.extend(path.iter().map(|s| s.to_str()).flatten());
        }
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

pub trait MutateUrl: AsMut<DriveUrl> + AsRef<DriveUrl> {
    fn append_query_pair(&mut self, key: &str, value: &str) {
        self.as_mut()
            .append_query_pair(key.as_ref(), value.as_ref());
    }

    fn endpoint(&mut self, de: DriveEndPoint) {
        let mut vec: Vec<&str> = de.as_str().split('/').collect();
        vec.retain(|s| !s.eq(&""));
        self.as_mut().extend_path(&vec);
    }

    fn event(&mut self, event: DriveEvent) {
        if !event.as_str().is_empty() {
            self.as_mut().extend_path(&[event.as_str()]);
        }
    }

    fn count(&mut self, value: &str) {
        self.as_mut().append_query_pair("count", value);
    }

    fn select(&mut self, value: &[&str]) {
        let s = value.join(" ");
        self.as_mut().append_query_pair("select", &s);
    }

    fn expand(&mut self, value: &[&str]) {
        let s = value.join(" ");
        self.as_mut().append_query_pair("expand", &s);
    }

    fn filter(&mut self, value: &[&str]) {
        let s = value.join(",");
        self.as_mut().append_query_pair("filter", &s);
    }

    fn order_by(&mut self, value: &[&str]) {
        let s = value.join(" ");
        self.as_mut().append_query_pair("orderby", &s);
    }

    fn search(&mut self, value: &str) {
        self.as_mut().append_query_pair("search", value);
    }

    fn format(&mut self, value: &str) {
        self.as_mut().append_query_pair("format", value);
    }

    fn skip(&mut self, value: &str) {
        self.as_mut().append_query_pair("skip", value.as_ref());
    }

    fn top(&mut self, value: &str) {
        self.as_mut().append_query_pair("top", value.as_ref());
    }
}
