use crate::calendar::CalendarRequest;
use crate::contacts::ContactsRequest;
use crate::drive::DriveRequest;
use crate::http::{DeltaRequest, GraphRequest, IntoResponse};
use crate::http::{GraphRequestBuilder, GraphResponse};
use crate::mail::MailRequest;
use crate::onenote::OnenoteRequest;
use crate::types::{boolresponse::BoolResponse, collection::Collection, content::Content};
use crate::url::GraphUrl;
use crate::{GRAPH_URL, GRAPH_URL_BETA};
use graph_error::GraphFailure;
use graph_oauth::oauth::{AccessToken, OAuth};
use graph_rs_types::entitytypes::{
    Conversation, ConversationThread, DirectoryObject, Drive, Event, FieldValueSet, Group,
    GroupLifecyclePolicy, ItemActivityStat, ItemAnalytics, List, ListItem, ListItemVersion,
    ProfilePhoto, Site, User, UserSettings
};
use handlebars::*;
use reqwest::header::{HeaderValue, ACCEPT};
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
    builder: RefCell<GraphRequestBuilder>,
    registry: RefCell<Handlebars>,
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
        let mut request = GraphRequest::default();
        request.set_token(token);
        Graph {
            request: RefCell::new(request),
            builder: RefCell::new(GraphRequestBuilder::new(
                GraphUrl::from_str(GRAPH_URL).unwrap(),
            )),
            registry: RefCell::new(Handlebars::new()),
            is_v1: Cell::new(true),
        }
    }

    /// Use the v1.0 Graph API
    pub fn v1(&'a self) -> Identify<'a> {
        self.is_v1.set(true);
        self.builder().as_mut().replace(GRAPH_URL).unwrap();
        Identify { client: &self }
    }

    /// Use the Graph beta API
    pub fn beta(&'a self) -> Identify<'a> {
        self.is_v1.set(false);
        self.builder().as_mut().replace(GRAPH_URL_BETA).unwrap();
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

    pub fn set_token(&self, token: &str) {
        self.request().set_token(token);
    }

    pub(crate) fn request(&self) -> RefMut<GraphRequest> {
        self.request.borrow_mut()
    }

    pub(crate) fn builder(&self) -> RefMut<GraphRequestBuilder> {
        self.builder.borrow_mut()
    }

    pub(crate) fn take_builder(&self) -> GraphRequestBuilder {
        self.builder.replace(GraphRequestBuilder::new(
            GraphUrl::from_str(GRAPH_URL).unwrap(),
        ))
    }

    pub(crate) fn registry(&self) -> RefMut<Handlebars> {
        self.registry.borrow_mut()
    }

    pub fn url_ref<F>(&self, f: F)
    where
        F: Fn(&GraphUrl),
    {
        f(&self.builder.borrow().as_ref())
    }

    pub fn url_mut<F>(&self, f: F)
    where
        F: Fn(&mut GraphUrl),
    {
        f(&mut self.builder().as_mut())
    }

    pub fn fn_mut_url<F>(&self, mut f: F)
    where
        F: FnMut(&mut GraphUrl),
    {
        f(&mut self.builder().as_mut())
    }
}

impl From<&str> for Graph {
    fn from(token: &str) -> Self {
        Graph::new(token)
    }
}

impl From<String> for Graph {
    fn from(token: String) -> Self {
        let mut request = GraphRequest::default();
        request.set_token(token.as_ref());
        Graph {
            request: RefCell::new(request),
            builder: RefCell::new(GraphRequestBuilder::new(
                GraphUrl::from_str(GRAPH_URL).unwrap(),
            )),
            registry: RefCell::new(Handlebars::new()),
            is_v1: Cell::new(true),
        }
    }
}

impl From<&AccessToken> for Graph {
    fn from(token: &AccessToken) -> Self {
        let mut request = GraphRequest::default();
        request.set_token(token.bearer_token());
        Graph {
            request: RefCell::new(request),
            builder: RefCell::new(GraphRequestBuilder::new(
                GraphUrl::from_str(GRAPH_URL).unwrap(),
            )),
            registry: RefCell::new(Handlebars::new()),
            is_v1: Cell::new(true),
        }
    }
}

impl TryFrom<&OAuth> for Graph {
    type Error = GraphFailure;

    fn try_from(oauth: &OAuth) -> Result<Self, Self::Error> {
        let access_token = oauth.get_access_token()?;
        Ok(Graph::from(&access_token))
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

    pub fn batch<B: serde::Serialize>(
        &self,
        batch: &B,
    ) -> IntoResponse<'a, IdentifyCommon, DeltaRequest> {
        self.client
            .builder()
            .set_method(Method::POST)
            .header(ACCEPT, HeaderValue::from_static("application/json"))
            .set_body(serde_json::to_string(batch).unwrap());
        render_path!(
            self.client,
            "$batch",
            &serde_json::json!({})
        );
        IntoResponse::new(self.client)
    }
}

register_ident_client!(IdentMe,);
register_ident_client!(IdentDrives,);
register_ident_client!(IdentSites,);
register_ident_client!(IdentGroups,);
register_ident_client!(IdentUsers,);

impl<'a, I> IdentMe<'a, I> {
    get!( get, User => "me" );
    get!( list_events, Collection<Event> => "me/events" );
    get!( settings, UserSettings => "me/settings" );
    patch!( [ update_settings, UserSettings => "me/settings" ] );
}

impl<'a, I> IdentDrives<'a, I> {
    get!( get, Drive => "drive/{{RID}}" );
}

impl<'a, I> IdentSites<'a, I> {
    get!( get, Site => "sites/{{RID}}" );
    get!( list_subsites, Collection<Site> => "sites/{{RID}}/sites" );
    get!( root, Site => "sites/root" );
    get!( | root_tenant, Site => "sites/{{id}}" );
    get!( analytics, ItemAnalytics => "sites/{{RID}}/analytics" );
    get!( | item_analytics, ItemAnalytics => "sites/{{RID}}/items/{{id}}/analytics" );
    get!( | list_item_versions, ListItemVersion => "sites/{{RID}}/items/{{id}}/versions" );

    pub fn lists(&'a self) -> SiteListRequest<'a, I> {
        SiteListRequest::new(self.client)
    }

    pub fn activities_by_interval(
        &'a self,
        start: &str,
        end: Option<&str>,
        interval: &str,
    ) -> IntoResponse<'a, I, ItemActivityStat> {
        self.client.builder().set_method(Method::GET);

        if let Some(end) = end {
            let interval = format!(
                "sites/{{{{RID}}}}/getActivitiesByInterval(startDateTime='{}',endDateTime='{}',interval='{}')",
                start,
                end,
                interval
            );
            render_path!(self.client, &interval);
        } else {
            let interval = format!(
                "sites/{{{{RID}}}}/getActivitiesByInterval(startDateTime='{}',interval='{}')",
                start, interval
            );
            render_path!(self.client, &interval);
        }
        IntoResponse::new(self.client)
    }
}

register_client!(SiteListRequest,);

impl<'a, I> SiteListRequest<'a, I> {
    get!( list, Collection<List> => "sites/{{RID}}/lists" );
    get!( | get, List => "sites/{{RID}}/lists/{{id}}" );
    post!( [ create, List => "sites/{{RID}}/lists" ] );

    pub fn items(&'a self) -> SiteListItemRequest<'a, I> {
        SiteListItemRequest::new(self.client)
    }
}

register_client!(SiteListItemRequest,);

impl<'a, I> SiteListItemRequest<'a, I> {
    get!( | list, Collection<ListItem> => "sites/{{RID}}/lists/{{id}}/items" );
    get!( || get, ListItem => "sites/{{RID}}/lists/{{id}}/items/{{id2}}" );
    get!( || analytics, ItemAnalytics => "sites/{{RID}}/lists/{{id}}/items/{{id2}}/analytics" );
    get!( || list_versions, ListItemVersion => "sites/{{RID}}/lists/{{id}}/items/{{id2}}/versions" );
    post!( [ | create, ListItem => "sites/{{RID}}/lists/{{id}}/items" ] );
    patch!( [ || update, FieldValueSet => "sites/{{RID}}/lists/{{id}}/items/{{id2}}" ] );
    patch!( [ || update_columns, FieldValueSet => "sites/{{RID}}/lists/{{id}}/items/{{id2}}/fields" ] );
    delete!( || delete, GraphResponse<Content> => "sites/{{RID}}/lists/{{id}}/items/{{id2}}" );
}

impl<'a, I> IdentGroups<'a, I> {
    get!( list, Collection<Group> => "groups" );
    get!( | get, Group => "groups/{{RID}}" );
    get!( delta, DeltaRequest => "groups/delta" );
    get!( list_events, Collection<Event> => "groups/{{RID}}/events" );
    get!( list_lifecycle_policies, Collection<GroupLifecyclePolicy> => "groups/{{RID}}/groupLifecyclePolicies" );
    get!( member_of, Collection<DirectoryObject> => "groups/{{RID}}/memberOf" );
    get!( transitive_member_of, Collection<DirectoryObject> => "groups/{{RID}}/transitiveMemberOf" );
    get!( list_members, Collection<DirectoryObject> => "groups/{{RID}}/members"  );
    get!( list_transitive_members, Collection<DirectoryObject> => "groups/{{RID}}/transitiveMembers" );
    get!( owners, Collection<User> => "groups/{{RID}}/owners" );
    get!( list_photos, Collection<ProfilePhoto> => "groups/{{RID}}/photos" );
    get!( root_site, Collection<ProfilePhoto> => "groups/{{RID}}/sites/root" );
    post!( [ create, Group => "groups" ] );
    post!( add_favorite, GraphResponse<Content> => "groups/{{RID}}/addFavorite" );
    post!( [ add_member, GraphResponse<Content> => "groups/{{RID}}/members/$ref" ] );
    post!( [ add_owner, GraphResponse<Content> => "groups/{{RID}}/owners/$ref" ] );
    post!( [ check_member_groups, Collection<String> => "groups/{{RID}}/checkMemberGroups" ] );
    post!( [ member_groups, Collection<String> => "groups/{{RID}}/getMemberGroups" ] );
    post!( [ member_objects, Collection<String> => "groups/{{RID}}/getMemberObjects" ] );
    post!( remove_favorite, GraphResponse<Content> => "groups/{{RID}}/removeFavorite" );
    post!( renew, GraphResponse<Content> => "groups/{{RID}}/renew" );
    post!( reset_unseen_count, GraphResponse<Content> => "groups/{{RID}}/resetUnseenCount" );
    post!( subscribe_by_mail, GraphResponse<Content> => "groups/{{RID}}/subscribeByMail" );
    post!( unsubscribe_by_mail, GraphResponse<Content> => "groups/{{RID}}/unsubscribeByMail" );
    post!( [ validate_properties, GraphResponse<Content> => "groups/{{RID}}/validateProperties" ] );
    patch!( [ update, Group => "groups/{{RID}}" ] );
    delete!( delete, GraphResponse<Content> => "groups/{{RID}}" );
    delete!( | remove_member, GraphResponse<Content> => "groups/{{RID}}/members/{{id}}/$ref" );
    delete!( | remove_owner, GraphResponse<Content> => "groups/{{RID}}/owners/{{id}}/$ref" );

    pub fn conversations(&self) -> GroupConversationRequest<'a, I> {
        GroupConversationRequest::new(self.client)
    }

    pub fn group_lifecycle_policies(&self) -> GroupLifecyclePolicyRequest<'a, I> {
        GroupLifecyclePolicyRequest::new(self.client)
    }
}

register_client!(
    GroupLifecyclePolicyRequest,
    glp => "groupLifecyclePolicies",
);

impl<'a, I> GroupLifecyclePolicyRequest<'a, I> {
    get!( list, Collection<GroupLifecyclePolicy> => "{{glp}}" );
    get!( | get, Collection<GroupLifecyclePolicy> => "{{glp}}/{{id}}" );
    post!( [ create, GroupLifecyclePolicy => "{{glp}}" ] );
    post!( [ | add_group, BoolResponse => "{{glp}}/{{id}}/addGroup" ] );
    post!( [ | remove_group, BoolResponse =>  "{{glp}}/{{id}}/removeGroup" ] );
    patch!( [ | update, GroupLifecyclePolicy => "{{glp}}/{{id}}" ] );
    patch!( | delete, GraphResponse<Content> => "{{glp}}/{{id}}" );
}

register_client!(
    GroupConversationRequest,
    co => "conversations",
);

impl<'a, I> GroupConversationRequest<'a, I> {
    get!( list, Collection<Conversation> => "groups/{{RID}}/{{co}}" );
    get!( | list_threads, Collection<ConversationThread> => "groups/{{RID}}/{{co}}/{{id}}/threads" );
    get!( list_accepted_senders, Collection<DirectoryObject> => "groups/{{RID}}/acceptedSenders" );
    get!( | get, Conversation => "groups/{{RID}}/{{co}}/{{id}}" );
    post!( [ create, Conversation => "groups/{{RID}}/{{co}}" ] );
    post!( [ | create_thread, ConversationThread => "groups/{{RID}}/{{co}}/{{id}}/threads" ] );
    post!( [ create_accepted_sender, GraphResponse<Content> => "groups/{{RID}}/acceptedSenders/$ref" ] );
    delete!( | delete, GraphResponse<Content> => "groups/{{RID}}/{{co}}/{{id}}" );
}

impl<'a, I> IdentUsers<'a, I> {
    get!( get, User => "users/{{RID}}" );
    get!( settings, UserSettings => "users/{{RID}}/settings" );
    get!( list, Collection<User> => "users" );
    get!( list_events, Collection<Event> => "users/{{RID}}/events" );
    get!( | list_joined_group_photos, Collection<ProfilePhoto> => "users/{{RID}}/joinedGroups/{{id}}/photos" );
    post!( [ create, User => "users" ] );
    post!( delta, DeltaRequest => "users" );
    patch!( [ update, GraphResponse<Content> => "users/{{RID}}" ] );
    patch!( [ update_settings, UserSettings => "users/{{RID}}/settings" ] );
    delete!( delete, GraphResponse<Content> => "users/{{RID}}" );
}
