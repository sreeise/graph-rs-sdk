use crate::client::Ident;
use std::cmp::Ordering;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialOrd, Eq, PartialEq)]
pub enum UrlOrdering {
    // me, drives, sites, users, groups
    Ident(Ident),
    // The id for drives, sites, users, and groups.
    ResourceId(String),
    // The normal route to a resource. For me paths this will be right after
    // the me and for others it will be between the resource id and item id.
    ItemPath(String),
    // Before an item id or path.
    RootOrItem(String),
    // The item id for a resource.
    Id(String),
    // Setting FileName will cause the url to be
    // formatted without brackets unlike a path.
    FileName(String),
    // Formatted for paths in OneDrive with brackets.
    Path(PathBuf),
    // The very last content in the path of the url.
    Last(String),
}

impl Ord for UrlOrdering {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            UrlOrdering::Ident(_) => match other {
                UrlOrdering::Ident(_) => Ordering::Equal,
                UrlOrdering::ResourceId(_) => Ordering::Greater,
                UrlOrdering::ItemPath(_) => Ordering::Greater,
                UrlOrdering::Id(_) => Ordering::Greater,
                UrlOrdering::Path(_) => Ordering::Greater,
                UrlOrdering::FileName(_) => Ordering::Greater,
                UrlOrdering::Last(_) => Ordering::Greater,
                UrlOrdering::RootOrItem(_) => Ordering::Greater,
            },
            UrlOrdering::ResourceId(_) => match other {
                UrlOrdering::Ident(_) => Ordering::Less,
                UrlOrdering::ResourceId(_) => Ordering::Equal,
                UrlOrdering::ItemPath(_) => Ordering::Greater,
                UrlOrdering::Id(_) => Ordering::Greater,
                UrlOrdering::Path(_) => Ordering::Greater,
                UrlOrdering::FileName(_) => Ordering::Greater,
                UrlOrdering::Last(_) => Ordering::Greater,
                UrlOrdering::RootOrItem(_) => Ordering::Greater,
            },
            UrlOrdering::Id(_) => match other {
                UrlOrdering::Ident(_) => Ordering::Less,
                UrlOrdering::ResourceId(_) => Ordering::Less,
                UrlOrdering::ItemPath(_) => Ordering::Less,
                UrlOrdering::Id(_) => Ordering::Equal,
                UrlOrdering::Path(_) => Ordering::Less,
                UrlOrdering::FileName(_) => Ordering::Greater,
                UrlOrdering::Last(_) => Ordering::Greater,
                UrlOrdering::RootOrItem(_) => Ordering::Greater,
            },
            UrlOrdering::Last(_) => match other {
                UrlOrdering::Ident(_) => Ordering::Less,
                UrlOrdering::ResourceId(_) => Ordering::Less,
                UrlOrdering::ItemPath(_) => Ordering::Less,
                UrlOrdering::Id(_) => Ordering::Less,
                UrlOrdering::Path(_) => Ordering::Less,
                UrlOrdering::FileName(_) => Ordering::Less,
                UrlOrdering::Last(_) => Ordering::Equal,
                UrlOrdering::RootOrItem(_) => Ordering::Less,
            },
            UrlOrdering::ItemPath(_) => match other {
                UrlOrdering::Ident(_) => Ordering::Less,
                UrlOrdering::ResourceId(_) => Ordering::Less,
                UrlOrdering::ItemPath(_) => Ordering::Equal,
                UrlOrdering::Id(_) => Ordering::Greater,
                UrlOrdering::Path(_) => Ordering::Greater,
                UrlOrdering::Last(_) => Ordering::Greater,
                UrlOrdering::RootOrItem(_) => Ordering::Greater,
                UrlOrdering::FileName(_) => Ordering::Greater,
            },
            UrlOrdering::Path(_) => match other {
                UrlOrdering::Ident(_) => Ordering::Less,
                UrlOrdering::ResourceId(_) => Ordering::Less,
                UrlOrdering::ItemPath(_) => Ordering::Less,
                UrlOrdering::Id(_) => Ordering::Less,
                UrlOrdering::Path(_) => Ordering::Equal,
                UrlOrdering::Last(_) => Ordering::Greater,
                UrlOrdering::RootOrItem(_) => Ordering::Less,
                UrlOrdering::FileName(_) => Ordering::Equal,
            },
            UrlOrdering::RootOrItem(_) => match other {
                UrlOrdering::Ident(_) => Ordering::Less,
                UrlOrdering::ResourceId(_) => Ordering::Less,
                UrlOrdering::ItemPath(_) => Ordering::Less,
                UrlOrdering::Id(_) => Ordering::Greater,
                UrlOrdering::Path(_) => Ordering::Greater,
                UrlOrdering::Last(_) => Ordering::Greater,
                UrlOrdering::RootOrItem(_) => Ordering::Equal,
                UrlOrdering::FileName(_) => Ordering::Greater,
            },
            UrlOrdering::FileName(_) => match other {
                UrlOrdering::Ident(_) => Ordering::Less,
                UrlOrdering::ResourceId(_) => Ordering::Less,
                UrlOrdering::ItemPath(_) => Ordering::Less,
                UrlOrdering::Id(_) => Ordering::Less,
                UrlOrdering::Path(_) => Ordering::Equal,
                UrlOrdering::Last(_) => Ordering::Greater,
                UrlOrdering::RootOrItem(_) => Ordering::Less,
                UrlOrdering::FileName(_) => Ordering::Equal,
            },
        }
    }
}

#[derive(Debug, Default)]
pub struct UrlOrdVec {
    pub(crate) ord: Vec<UrlOrdering>,
}

impl UrlOrdVec {
    pub fn insert(&mut self, ord: UrlOrdering) -> &mut Self {
        self.ord.push(ord);
        self
    }

    pub fn replace(&mut self, ord: UrlOrdering) -> &mut Self {
        self.ord.retain(|s| s.cmp(&ord) != Ordering::Equal);
        self.ord.push(ord);
        self
    }

    pub fn remove(&mut self, ord: UrlOrdering) -> &mut Self {
        self.ord.retain(|s| s.cmp(&ord) != Ordering::Equal);
        self
    }

    pub fn remove_ref(&mut self, ord: &UrlOrdering) -> &mut Self {
        self.ord.retain(|s| s.cmp(ord) != Ordering::Equal);
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        self.ord.clear();
        self
    }

    pub fn sort(&mut self) -> &mut Self {
        self.ord.sort();
        self
    }
}

impl Extend<UrlOrdering> for UrlOrdVec {
    fn extend<I: IntoIterator<Item = UrlOrdering>>(&mut self, iter: I) {
        self.ord.extend(iter);
    }
}

#[derive(Clone, Debug)]
pub enum FormatOrd {
    Insert(UrlOrdering),
    InsertEq(UrlOrdering, Ident),
    InsertNe(UrlOrdering, Ident),
    Replace(UrlOrdering),
    Remove(UrlOrdering),
}
