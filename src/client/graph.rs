use crate::drive::DriveRequest;
use crate::http::GraphRequest;
use crate::lists::ListRequest;
use crate::url::{GraphUrl, UrlOrdering};
use crate::users::UserRequest;
use crate::{GRAPH_URL, GRAPH_URL_BETA};
use graph_error::GraphFailure;
use graph_oauth::oauth::{AccessToken, OAuth};
use std::cell::{RefCell, RefMut};
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::str::FromStr;

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

pub struct Graph {
    pub(crate) request: RefCell<GraphRequest>,
    pub(crate) ident: RefCell<Ident>,
}

impl<'a> Graph {
    pub fn new(token: &str) -> Graph {
        let mut request = GraphRequest::from(GraphUrl::from_str(GRAPH_URL).unwrap());
        request.set_token(token);
        Graph {
            request: RefCell::new(request),
            ident: RefCell::new(Ident::Me),
        }
    }

    pub fn v1(&'a self) -> Identify<'a> {
        self.request().clear().as_mut().replace(GRAPH_URL).unwrap();
        Identify { client: &self }
    }

    pub fn beta(&'a self) -> Identify<'a> {
        self.request()
            .clear()
            .as_mut()
            .replace(GRAPH_URL_BETA)
            .unwrap();
        Identify { client: &self }
    }

    pub fn ident(&self) -> Ident {
        *self.ident.borrow()
    }

    pub(crate) fn request(&self) -> RefMut<GraphRequest> {
        self.request.borrow_mut()
    }

    pub(crate) fn set_ident(&self, ident: Ident) {
        self.ident.replace(ident);
    }

    pub fn format_ord(&self) {
        self.request().format_ord();
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
}

impl From<&str> for Graph {
    fn from(token: &str) -> Self {
        let mut request = GraphRequest::from(GraphUrl::from_str(GRAPH_URL).unwrap());
        request.set_token(token);
        Graph {
            request: RefCell::new(request),
            ident: RefCell::new(Ident::Me),
        }
    }
}

impl From<String> for Graph {
    fn from(token: String) -> Self {
        let mut request = GraphRequest::from(GraphUrl::from_str(GRAPH_URL).unwrap());
        request.set_token(token.as_ref());
        Graph {
            request: RefCell::new(request),
            ident: RefCell::new(Ident::Me),
        }
    }
}

impl From<&AccessToken> for Graph {
    fn from(token: &AccessToken) -> Self {
        let mut request = GraphRequest::from(GraphUrl::from_str(GRAPH_URL).unwrap());
        request.set_token(token.get_access_token());
        Graph {
            request: RefCell::new(request),
            ident: RefCell::new(Ident::Me),
        }
    }
}

impl TryFrom<&OAuth> for Graph {
    type Error = GraphFailure;

    fn try_from(oauth: &OAuth) -> Result<Self, Self::Error> {
        let access_token = oauth
            .get_access_token()
            .ok_or_else(|| GraphFailure::none_err("access_token"))?;
        let mut request = GraphRequest::from(GraphUrl::from_str(GRAPH_URL).unwrap());
        request.set_token(access_token.get_access_token());
        Ok(Graph {
            request: RefCell::new(request),
            ident: RefCell::new(Ident::Me),
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
        self.client.request().insert(UrlOrdering::Ident(Ident::Me));
        self.client.set_ident(Ident::Me);
        GraphPath::new(self.client)
    }

    pub fn drives(&self) -> GraphPath<'a, IdentifyCommon> {
        self.client
            .request()
            .insert(UrlOrdering::Ident(Ident::Drives));
        self.client.set_ident(Ident::Drives);
        GraphPath::new(self.client)
    }

    pub fn sites(&self) -> GraphPath<'a, IdentifyCommon> {
        self.client
            .request()
            .insert(UrlOrdering::Ident(Ident::Sites));
        self.client.set_ident(Ident::Sites);
        GraphPath::new(self.client)
    }

    pub fn groups(&self) -> GraphPath<'a, IdentifyCommon> {
        self.client
            .request()
            .insert(UrlOrdering::Ident(Ident::Groups));
        self.client.set_ident(Ident::Groups);
        GraphPath::new(self.client)
    }

    pub fn users(&self) -> GraphPath<'a, IdentifyCommon> {
        self.client
            .request()
            .insert(UrlOrdering::Ident(Ident::Users));
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

    pub fn user(&'a self) -> UserRequest<'a, I> {
        UserRequest::new(self.client)
    }
}
