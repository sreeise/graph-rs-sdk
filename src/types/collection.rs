use crate::types::delta::{DeltaLink, MetadataLink, NextLink};
use from_as::*;

/// A Collection stores a collection of items returned from the API such
/// as a collection of DriveItem's or User's.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromFile, AsFile)]
pub struct Collection<T> {
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
        next_link: Option<String>,
        delta_link: Option<String>,
        context: Option<String>,
        value: Option<Vec<T>>,
    ) -> Collection<T> {
        Collection {
            odata_next_link: next_link,
            odata_delta_link: delta_link,
            odata_context: context,
            value,
        }
    }

    pub fn odata_next_link(&self) -> Option<&String> {
        self.odata_next_link.as_ref()
    }

    pub fn odata_delta_link(&self) -> Option<&String> {
        self.odata_delta_link.as_ref()
    }

    pub fn odata_context(&self) -> Option<&String> {
        self.odata_context.as_ref()
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

    pub fn value(&self) -> Option<&Vec<T>> {
        self.value.as_ref()
    }

    pub fn value_mut(&mut self) -> &mut Option<Vec<T>> {
        &mut self.value
    }
}

impl<T> Eq for Collection<T> where T: std::cmp::PartialEq {}

impl<T> IntoIterator for Collection<T>
where
    T: Clone,
{
    type Item = T;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.value.unwrap_or_default().into_iter()
    }
}

impl<T> NextLink for Collection<T> {
    fn next_link(&self) -> Option<String> {
        self.odata_next_link.clone()
    }
}

impl<T> DeltaLink for Collection<T> {
    fn delta_link(&self) -> Option<String> {
        self.odata_delta_link.clone()
    }
}

impl<T> MetadataLink for Collection<T> {
    fn metadata_link(&self) -> Option<String> {
        self.odata_context.clone()
    }
}
