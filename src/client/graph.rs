use crate::calendar::CalendarRequest;
use crate::drive::DriveRequest;
use crate::http::GraphResponse;
use crate::http::{GraphRequest, IntoResponse};
use crate::lists::ListRequest;
use crate::mail::MailRequest;
use crate::onenote::OneNoteRequest;
use crate::types::collection::Collection;
use crate::url::GraphUrl;
use crate::{GRAPH_URL, GRAPH_URL_BETA};
use graph_error::GraphFailure;
use graph_oauth::oauth::{AccessToken, OAuth};
use graph_rs_types::entitytypes::{Drive, Group, Site, User};
use handlebars::*;
use reqwest::Method;
use std::cell::{Cell, RefCell, RefMut};
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

impl Default for Ident {
    fn default() -> Self {
        Ident::Me
    }
}

pub struct Graph {
    request: RefCell<GraphRequest>,
    is_v1: Cell<bool>,
}

impl<'a> Graph {
    /// Create a new client with an access token.
    ///
    /// # Example
    /// ```
    /// use graph_rs::client::Graph;
    ///
    /// let client = Graph::new("ACCESS_TOKEN");
    /// ```
    /// ```rust,ignore
    /// // Use the v1.0 API
    /// let drive_items: serde_json::Value = client
    ///     .v1()
    ///     .me()
    ///     .drive()
    ///     .root_children()
    ///     .json()?;
    /// ```
    pub fn new(token: &str) -> Graph {
        let mut request = GraphRequest::from(GraphUrl::from_str(GRAPH_URL).unwrap());
        request.set_token(token);
        Graph {
            request: RefCell::new(request),
            is_v1: Cell::new(true),
        }
    }

    /// Use the v1.0 Graph API
    pub fn v1(&'a self) -> Identify<'a> {
        self.is_v1.set(true);
        self.request().as_mut().replace(GRAPH_URL).unwrap();
        Identify { client: &self }
    }

    /// Use the Graph beta API
    pub fn beta(&'a self) -> Identify<'a> {
        self.is_v1.set(false);
        self.request().as_mut().replace(GRAPH_URL_BETA).unwrap();
        Identify { client: &self }
    }

    pub fn is_v1(&self) -> bool {
        self.is_v1.get()
    }

    pub fn is_beta(&self) -> bool {
        !self.is_v1.get()
    }

    pub fn ident(&self) -> Ident {
        self.request.borrow().ident()
    }

    pub(crate) fn request(&self) -> RefMut<GraphRequest> {
        self.request.borrow_mut()
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
            is_v1: Cell::new(true),
        }
    }
}

impl From<String> for Graph {
    fn from(token: String) -> Self {
        let mut request = GraphRequest::from(GraphUrl::from_str(GRAPH_URL).unwrap());
        request.set_token(token.as_ref());
        Graph {
            request: RefCell::new(request),
            is_v1: Cell::new(true),
        }
    }
}

impl From<&AccessToken> for Graph {
    fn from(token: &AccessToken) -> Self {
        let mut request = GraphRequest::from(GraphUrl::from_str(GRAPH_URL).unwrap());
        request.set_token(token.bearer_token());
        Graph {
            request: RefCell::new(request),
            is_v1: Cell::new(true),
        }
    }
}

impl TryFrom<&OAuth> for Graph {
    type Error = GraphFailure;

    fn try_from(oauth: &OAuth) -> Result<Self, Self::Error> {
        let access_token = oauth.get_access_token()?;
        let mut request = GraphRequest::from(GraphUrl::from_str(GRAPH_URL).unwrap());
        request.set_token(access_token.bearer_token());
        Ok(Graph {
            request: RefCell::new(request),
            is_v1: Cell::new(true),
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
    /// Select the me endpoint.
    pub fn me(&self) -> IdentMe<'a, IdentifyMe> {
        self.client.request().set_ident(Ident::Me);
        IdentMe::new("", self.client)
    }

    /// Select the drives endpoint.
    pub fn drives<S: AsRef<str>>(&self, id: S) -> IdentDrives<'a, IdentifyCommon> {
        self.client.request().set_ident(Ident::Drives);
        IdentDrives::new(id.as_ref(), self.client)
    }

    /// Select the sites endpoint.
    pub fn sites<S: AsRef<str>>(&self, id: S) -> IdentSites<'a, IdentifyCommon> {
        self.client.request().set_ident(Ident::Sites);
        IdentSites::new(id.as_ref(), self.client)
    }

    /// Select the groups endpoint.
    pub fn groups<S: AsRef<str>>(&self, id: S) -> IdentGroups<'a, IdentifyCommon> {
        self.client.request().set_ident(Ident::Groups);
        IdentGroups::new(id.as_ref(), self.client)
    }

    /// Select the users endpoint.
    pub fn users<S: AsRef<str>>(&self, id: S) -> IdentUsers<'a, IdentifyCommon> {
        self.client.request().set_ident(Ident::Users);
        IdentUsers::new(id.as_ref(), self.client)
    }
}

register_ident_client!(IdentMe,);

impl<'a, I> IdentMe<'a, I> {
    get!( get, User => "me" );
}

register_ident_client!(IdentDrives,);

impl<'a, I> IdentDrives<'a, I> {
    get!( get, Drive => "drive/{{RID}}" );
}

register_ident_client!(IdentSites,);

impl<'a, I> IdentSites<'a, I> {
    get!( get, Site => "sites/{{RID}}" );
}

register_ident_client!(IdentGroups,);

impl<'a, I> IdentGroups<'a, I> {
    get!( get, Group => "groups/{{RID}}" );
}

register_ident_client!(IdentUsers,);

impl<'a, I> IdentUsers<'a, I> {
    get!( get, User => "users/{{RID}}" );
    get!( list, Collection<User> => "users" );
    post!( [ create, User => "users" ] );
    patch!( [ update, GraphResponse<()> => "users/{{RID}}" ] );
    delete!( delete, GraphResponse<()> => "users/{{RID}}" );
}
