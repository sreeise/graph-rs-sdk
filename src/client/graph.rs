use crate::drive::client::{DriveRequest, ListRequest};
use crate::drive::{GRAPH_URL, GRAPH_URL_BETA};
use crate::http::Request;
use crate::http::{Download, FetchClient};
use crate::types::statusresponse::StatusResponse;
use crate::url::GraphUrl;
use graph_error::{GraphFailure, GraphResult};
use graph_oauth::oauth::{AccessToken, OAuth};
use reqwest::header::{HeaderValue, IntoHeaderName};
use reqwest::Method;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::ffi::OsString;
use std::fs::File;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use url::Url;

#[derive(Clone, Debug, PartialOrd, Eq, PartialEq)]
pub enum UrlOrdering {
    // me, drives, sites, users, groups
    Ident(Ident),
    // Specifies the place to look for a resource such as drive or lists.
    PathIdent(PathIdent),
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
    // The very last content in the url.
    Last(String),
}

impl Ord for UrlOrdering {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            UrlOrdering::Ident(_) => match other {
                UrlOrdering::Ident(_) => Ordering::Equal,
                UrlOrdering::PathIdent(_) => Ordering::Greater,
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
                UrlOrdering::PathIdent(_) => Ordering::Greater,
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
                UrlOrdering::PathIdent(_) => Ordering::Less,
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
                UrlOrdering::PathIdent(_) => Ordering::Less,
                UrlOrdering::ResourceId(_) => Ordering::Less,
                UrlOrdering::ItemPath(_) => Ordering::Less,
                UrlOrdering::Id(_) => Ordering::Less,
                UrlOrdering::Path(_) => Ordering::Less,
                UrlOrdering::FileName(_) => Ordering::Less,
                UrlOrdering::Last(_) => Ordering::Equal,
                UrlOrdering::RootOrItem(_) => Ordering::Less,
            },
            UrlOrdering::PathIdent(_) => match other {
                UrlOrdering::Ident(_) => Ordering::Less,
                UrlOrdering::PathIdent(_) => Ordering::Equal,
                UrlOrdering::ResourceId(_) => Ordering::Less,
                UrlOrdering::ItemPath(_) => Ordering::Less,
                UrlOrdering::Id(_) => Ordering::Greater,
                UrlOrdering::Path(_) => Ordering::Greater,
                UrlOrdering::Last(_) => Ordering::Greater,
                UrlOrdering::RootOrItem(_) => Ordering::Greater,
                UrlOrdering::FileName(_) => Ordering::Greater,
            },
            UrlOrdering::ItemPath(_) => match other {
                UrlOrdering::Ident(_) => Ordering::Less,
                UrlOrdering::PathIdent(_) => Ordering::Less,
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
                UrlOrdering::PathIdent(_) => Ordering::Less,
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
                UrlOrdering::PathIdent(_) => Ordering::Less,
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
                UrlOrdering::PathIdent(_) => Ordering::Less,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Ident {
    Me,
    Drives,
    Sites,
    Groups,
    Users,
}

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        match self {
            Ident::Me => "me",
            Ident::Drives => "drives",
            Ident::Sites => "sites",
            Ident::Groups => "groups",
            Ident::Users => "users",
        }
    }
}

impl Default for Ident {
    fn default() -> Self {
        Ident::Me
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PathIdent {
    Drive,
    List,
}

impl AsRef<str> for PathIdent {
    fn as_ref(&self) -> &str {
        match self {
            PathIdent::Drive => "drive",
            PathIdent::List => "lists",
        }
    }
}

pub struct Graph {
    pub(crate) request: RefCell<Request>,
    pub(crate) ident: RefCell<Ident>,
    pub(crate) url_ord: RefCell<Vec<UrlOrdering>>,
}

impl Graph {
    pub fn ident(&self) -> Ident {
        *self.ident.borrow()
    }

    pub(crate) fn set_ident(&self, ident: Ident) {
        self.ident.replace(ident);
    }
}

impl<'a> Graph {
    pub fn new(token: &str) -> Graph {
        let mut request = Request::from(GraphUrl::from_str(GRAPH_URL).unwrap());
        request.set_token(token);
        Graph {
            request: RefCell::new(request),
            ident: RefCell::new(Ident::Me),
            url_ord: RefCell::new(Default::default()),
        }
    }

    pub fn v1(&'a self) -> Identify<'a> {
        self.clear_ord();
        self.request
            .borrow_mut()
            .as_mut()
            .replace(GRAPH_URL)
            .unwrap();
        Identify { client: &self }
    }

    pub fn beta(&'a self) -> Identify<'a> {
        self.clear_ord();
        self.request
            .borrow_mut()
            .as_mut()
            .replace(GRAPH_URL_BETA)
            .unwrap();
        Identify { client: &self }
    }

    pub(crate) fn set_method(&self, method: Method) -> &Self {
        self.request.borrow_mut().set_method(method);
        self
    }

    pub(crate) fn body<B: Into<reqwest::Body>>(&self, body: B) -> &Self {
        self.request.borrow_mut().set_body(Some(body.into()));
        self
    }

    pub(crate) fn header<K: IntoHeaderName>(&self, key: K, value: HeaderValue) -> &Self {
        self.request.borrow_mut().header(key, value);
        self
    }

    pub(crate) fn set_file(&self, file: OsString) -> &Self {
        self.body(File::open(file).unwrap());
        self
    }

    pub(crate) fn set_upload_session<P: AsRef<Path>>(&self, file: P) -> &Self {
        self.request
            .borrow_mut()
            .set_upload_session_file(file.as_ref().to_path_buf().into_os_string());
        self
    }

    pub(crate) fn set_download_path(&self, path: PathBuf) -> &Self {
        self.request
            .borrow_mut()
            .download_request
            .set_directory(path);
        self
    }

    pub(crate) fn set_direct_download(&self, value: bool, url: &str) -> &Self {
        self.request
            .borrow_mut()
            .download_request
            .set_direct_download(value);
        self.request
            .borrow_mut()
            .set_url(GraphUrl::from(Url::parse(url).unwrap()));
        self
    }

    pub(crate) fn download_client(&self) -> GraphResult<FetchClient> {
        self.request.borrow_mut().download()
    }

    pub(crate) fn send<T>(&self) -> GraphResult<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        self.request.borrow_mut().json()
    }

    pub(crate) fn status_response(&self) -> GraphResult<StatusResponse> {
        self.request.borrow_mut().status_response()
    }

    pub(crate) fn insert_ord(&self, ord: UrlOrdering) -> &Self {
        self.url_ord.borrow_mut().push(ord);
        self
    }

    pub(crate) fn replace_ord(&self, ord: UrlOrdering) -> &Self {
        let mut url_ord = self.url_ord.borrow_mut();
        url_ord.retain(|s| s.cmp(&ord) != Ordering::Equal);
        url_ord.push(ord);
        self
    }

    pub(crate) fn remove_ord(&self, ord: UrlOrdering) -> &Self {
        let mut url_ord = self.url_ord.borrow_mut();
        url_ord.retain(|s| s.cmp(&ord) != Ordering::Equal);
        self
    }

    pub(crate) fn clear_ord(&self) {
        self.url_ord.borrow_mut().clear();
    }

    pub fn format(&self) -> &Self {
        let mut url_ord = self.url_ord.borrow_mut();
        url_ord.sort();
        self.url_mut(|url| {
            for url_ord in url_ord.iter() {
                match url_ord {
                    UrlOrdering::Ident(ident) => {
                        url.extend_path(&[ident]);
                    },
                    UrlOrdering::PathIdent(ident) => {
                        url.extend_path(&[ident]);
                    },
                    UrlOrdering::ResourceId(id) => {
                        url.extend_path(&[id.as_str()]);
                    },
                    UrlOrdering::ItemPath(s) => {
                        let mut v: Vec<&str> = s.split('/').collect();
                        v.retain(|s| !s.is_empty());
                        url.extend_path(&v);
                    },
                    UrlOrdering::Id(id) => {
                        url.extend_path(&[id.as_str()]);
                    },
                    UrlOrdering::Path(p) => {
                        url.format_path(p.as_path());
                    },
                    UrlOrdering::Last(s) => {
                        let mut v: Vec<&str> = s.split('/').collect();
                        v.retain(|s| !s.is_empty());
                        url.extend_path(&v);
                    },
                    UrlOrdering::RootOrItem(s) => {
                        url.extend_path(&[s.as_str()]);
                    },
                    UrlOrdering::FileName(s) => {
                        url.extend_path(&[s.as_str()]);
                    },
                }
            }
        });
        url_ord.clear();
        self
    }

    pub fn url_ref<F>(&self, f: F)
    where
        F: Fn(&GraphUrl),
    {
        f(&self.request.borrow().url)
    }

    pub fn url_mut<F>(&self, f: F)
    where
        F: Fn(&mut GraphUrl),
    {
        f(&mut self.request.borrow_mut().url)
    }

    pub fn fn_mut_url<F>(&self, mut f: F)
    where
        F: FnMut(&mut GraphUrl),
    {
        f(&mut self.request.borrow_mut().url)
    }

    pub fn select(&self, value: &[&str]) {
        self.request.borrow_mut().url.select(value);
        self.format();
    }

    pub fn expand(&self, value: &[&str]) {
        self.request.borrow_mut().url.expand(value);
        self.format();
    }

    pub fn filter(&self, value: &[&str]) {
        self.request.borrow_mut().url.filter(value);
        self.format();
    }

    pub fn order_by(&self, value: &[&str]) {
        self.request.borrow_mut().url.order_by(value);
        self.format();
    }

    pub fn top(&self, value: &str) {
        self.request.borrow_mut().url.top(value);
        self.format();
    }
}

impl From<&str> for Graph {
    fn from(token: &str) -> Self {
        let mut request = Request::from(GraphUrl::from_str(GRAPH_URL).unwrap());
        request.set_token(token);
        Graph {
            request: RefCell::new(request),
            ident: RefCell::new(Ident::Me),
            url_ord: RefCell::new(Default::default()),
        }
    }
}

impl From<&AccessToken> for Graph {
    fn from(token: &AccessToken) -> Self {
        let mut request = Request::from(GraphUrl::from_str(GRAPH_URL).unwrap());
        request.set_token(token.get_access_token());
        Graph {
            request: RefCell::new(request),
            ident: RefCell::new(Ident::Me),
            url_ord: RefCell::new(Default::default()),
        }
    }
}

impl TryFrom<&OAuth> for Graph {
    type Error = GraphFailure;

    fn try_from(oauth: &OAuth) -> Result<Self, Self::Error> {
        let access_token = oauth
            .get_access_token()
            .ok_or_else(|| GraphFailure::none_err("access_token"))?;
        let token = access_token.get_access_token();
        let mut request = Request::from(GraphUrl::from_str(GRAPH_URL).unwrap());
        request.set_token(token);
        Ok(Graph {
            request: RefCell::new(request),
            ident: RefCell::new(Ident::Me),
            url_ord: RefCell::new(Default::default()),
        })
    }
}

pub struct Identify<'a> {
    client: &'a Graph,
}

// Constraint for the me only path.
pub struct IdentifyMe {}

// Constraint for the non-me paths.
pub struct IdentifyCommon {}

impl<'a> Identify<'a> {
    pub fn me(&self) -> GraphPath<'a, IdentifyMe> {
        self.client.insert_ord(UrlOrdering::Ident(Ident::Me));
        self.client.set_ident(Ident::Me);
        GraphPath::new(self.client)
    }

    pub fn drives(&self) -> GraphPath<'a, IdentifyCommon> {
        self.client.insert_ord(UrlOrdering::Ident(Ident::Drives));
        self.client.set_ident(Ident::Drives);
        GraphPath::new(self.client)
    }

    pub fn sites(&self) -> GraphPath<'a, IdentifyCommon> {
        self.client.insert_ord(UrlOrdering::Ident(Ident::Sites));
        self.client.set_ident(Ident::Sites);
        GraphPath::new(self.client)
    }

    pub fn groups(&self) -> GraphPath<'a, IdentifyCommon> {
        self.client.insert_ord(UrlOrdering::Ident(Ident::Groups));
        self.client.set_ident(Ident::Groups);
        GraphPath::new(self.client)
    }

    pub fn users(&self) -> GraphPath<'a, IdentifyCommon> {
        self.client.insert_ord(UrlOrdering::Ident(Ident::Users));
        self.client.set_ident(Ident::Users);
        GraphPath::new(self.client)
    }
}

pub struct GraphPath<'a, I> {
    client: &'a Graph,
    ident: PhantomData<I>,
}

impl<'a, I> GraphPath<'a, I> {
    pub fn new(client: &'a Graph) -> GraphPath<'a, I> {
        GraphPath {
            client,
            ident: PhantomData,
        }
    }

    pub fn drive(&'a self) -> DriveRequest<'a, I> {
        DriveRequest::new(self.client)
    }

    pub fn lists(&'a self) -> ListRequest<'a, I> {
        ListRequest::new(self.client)
    }
}
