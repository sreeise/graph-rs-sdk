use crate::attachments::{
    AttachmentRequest, ThreadConvoPostAttachmentRequest, ThreadPostAttachmentRequest,
};
use crate::calendar::CalendarRequest;
use crate::contacts::ContactsRequest;
use crate::drive::DriveRequest;
use crate::http::{GraphRequest, IntoResponse};
use crate::http::{GraphRequestBuilder, GraphResponse};
use crate::mail::MailRequest;
use crate::onenote::OnenoteRequest;
use crate::types::{
    boolresponse::BoolResponse, collection::Collection, content::Content, delta::DeltaRequest,
};
use crate::url::GraphUrl;
use crate::{GRAPH_URL, GRAPH_URL_BETA};
use graph_error::GraphFailure;
use graph_oauth::oauth::{AccessToken, OAuth};
use handlebars::*;
use reqwest::header::{HeaderValue, ACCEPT};
use reqwest::Method;
use std::cell::{Cell, RefCell, RefMut};
use std::convert::TryFrom;
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

    /// Check if the current host is v1.0.
    pub fn is_v1(&self) -> bool {
        self.is_v1.get()
    }

    /// Check if the current host is beta.
    pub fn is_beta(&self) -> bool {
        !self.is_v1.get()
    }

    pub fn ident(&self) -> Ident {
        self.request.borrow().ident()
    }

    /// Set the access token used for requests.
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

impl<'a> Identify<'a> {
    /// Select the me endpoint.
    pub fn me(&self) -> IdentMe<'a> {
        self.client.request().set_ident(Ident::Me);
        IdentMe::new("", self.client)
    }

    /// Select the drives endpoint.
    pub fn drives<S: AsRef<str>>(&self, id: S) -> IdentDrives<'a> {
        self.client.request().set_ident(Ident::Drives);
        IdentDrives::new(id.as_ref(), self.client)
    }

    /// Select the sites endpoint.
    pub fn sites<S: AsRef<str>>(&self, id: S) -> IdentSites<'a> {
        self.client.request().set_ident(Ident::Sites);
        IdentSites::new(id.as_ref(), self.client)
    }

    /// Select the groups endpoint.
    pub fn groups<S: AsRef<str>>(&self, id: S) -> IdentGroups<'a> {
        self.client.request().set_ident(Ident::Groups);
        IdentGroups::new(id.as_ref(), self.client)
    }

    /// Select the group lifecycle policies endpoint.
    pub fn group_lifecycle_policies<S: AsRef<str>>(
        &self,
        id: S,
    ) -> GroupLifecyclePolicyRequest<'a> {
        GroupLifecyclePolicyRequest::new(id.as_ref(), self.client)
    }

    /// Select the users endpoint.
    pub fn users<S: AsRef<str>>(&self, id: S) -> IdentUsers<'a> {
        self.client.request().set_ident(Ident::Users);
        IdentUsers::new(id.as_ref(), self.client)
    }

    pub fn batch<B: serde::Serialize>(
        &self,
        batch: &B,
    ) -> IntoResponse<'a, DeltaRequest<serde_json::Value>> {
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

impl<'a> IdentMe<'a> {
    get!( get, serde_json::Value => "me" );
    get!( list_events, Collection<serde_json::Value> => "me/events" );
    get!( settings, serde_json::Value => "me/settings" );
    patch!( [ update_settings, serde_json::Value => "me/settings" ] );
}

impl<'a> IdentDrives<'a> {
    get!( get, serde_json::Value => "drive/{{RID}}" );
}

impl<'a> IdentSites<'a> {
    get!( get, serde_json::Value => "sites/{{RID}}" );
    get!( list_subsites, Collection<serde_json::Value> => "sites/{{RID}}/sites" );
    get!( root, serde_json::Value => "sites/root" );
    get!( | root_tenant, serde_json::Value => "sites/{{id}}" );
    get!( analytics, serde_json::Value => "sites/{{RID}}/analytics" );
    get!( | item_analytics, serde_json::Value => "sites/{{RID}}/items/{{id}}/analytics" );
    get!( | list_item_versions, serde_json::Value => "sites/{{RID}}/items/{{id}}/versions" );

    pub fn lists(&'a self) -> SiteListRequest<'a> {
        SiteListRequest::new(self.client)
    }

    pub fn activities_by_interval(
        &'a self,
        start: &str,
        end: Option<&str>,
        interval: &str,
    ) -> IntoResponse<'a, serde_json::Value> {
        self.client.builder().set_method(Method::GET);
        if let Some(end) = end {
            render_path!(self.client, &format!(
                "sites/{{{{RID}}}}/getActivitiesByInterval(startDateTime='{}',endDateTime='{}',interval='{}')",
                start,
                end,
                interval
            ));
        } else {
            render_path!(
                self.client,
                &format!(
                    "sites/{{{{RID}}}}/getActivitiesByInterval(startDateTime='{}',interval='{}')",
                    start, interval
                )
            );
        }
        IntoResponse::new(self.client)
    }
}

register_client!(SiteListRequest,);

impl<'a> SiteListRequest<'a> {
    get!( list, Collection<serde_json::Value> => "sites/{{RID}}/lists" );
    get!( | get, serde_json::Value => "sites/{{RID}}/lists/{{id}}" );
    post!( [ create, serde_json::Value => "sites/{{RID}}/lists" ] );

    pub fn items(&'a self) -> SiteListItemRequest<'a> {
        SiteListItemRequest::new(self.client)
    }
}

register_client!(SiteListItemRequest,);

impl<'a> SiteListItemRequest<'a> {
    get!( | list, Collection<serde_json::Value> => "sites/{{RID}}/lists/{{id}}/items" );
    get!( || get, serde_json::Value => "sites/{{RID}}/lists/{{id}}/items/{{id2}}" );
    get!( || analytics, serde_json::Value => "sites/{{RID}}/lists/{{id}}/items/{{id2}}/analytics" );
    get!( || list_versions, serde_json::Value => "sites/{{RID}}/lists/{{id}}/items/{{id2}}/versions" );
    post!( [ | create, serde_json::Value => "sites/{{RID}}/lists/{{id}}/items" ] );
    patch!( [ || update, serde_json::Value => "sites/{{RID}}/lists/{{id}}/items/{{id2}}" ] );
    patch!( [ || update_columns, serde_json::Value => "sites/{{RID}}/lists/{{id}}/items/{{id2}}/fields" ] );
    delete!( || delete, GraphResponse<Content> => "sites/{{RID}}/lists/{{id}}/items/{{id2}}" );
}

impl<'a> IdentGroups<'a> {
    get!( list, Collection<serde_json::Value> => "groups" );
    get!( get, serde_json::Value => "groups/{{RID}}" );
    get!( delta, DeltaRequest<Collection<serde_json::Value>> => "groups/delta" );
    get!( list_events, Collection<serde_json::Value> => "groups/{{RID}}/events" );
    get!( list_lifecycle_policies, Collection<serde_json::Value> => "groups/{{RID}}/groupLifecyclePolicies" );
    get!( list_member_of, Collection<serde_json::Value> => "groups/{{RID}}/memberOf" );
    get!( list_transitive_member_of, Collection<serde_json::Value> => "groups/{{RID}}/transitiveMemberOf" );
    get!( list_members, Collection<serde_json::Value> => "groups/{{RID}}/members"  );
    get!( list_transitive_members, Collection<serde_json::Value> => "groups/{{RID}}/transitiveMembers" );
    get!( list_owners, Collection<serde_json::Value> => "groups/{{RID}}/owners" );
    get!( list_photos, Collection<serde_json::Value> => "groups/{{RID}}/photos" );
    get!( root_site, Collection<serde_json::Value> => "groups/{{RID}}/sites/root" );
    post!( [ create, serde_json::Value => "groups" ] );
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
    patch!( [ update, serde_json::Value => "groups/{{RID}}" ] );
    delete!( delete, GraphResponse<Content> => "groups/{{RID}}" );
    delete!( | remove_member, GraphResponse<Content> => "groups/{{RID}}/members/{{id}}/$ref" );
    delete!( | remove_owner, GraphResponse<Content> => "groups/{{RID}}/owners/{{id}}/$ref" );

    pub fn conversations(&self) -> GroupConversationRequest<'a> {
        GroupConversationRequest::new(self.client)
    }

    pub fn conversation_posts(&'a self) -> GroupConversationPostRequest<'a> {
        GroupConversationPostRequest::new(self.client)
    }

    pub fn thread_posts(&'a self) -> GroupThreadPostRequest<'a> {
        GroupThreadPostRequest::new(self.client)
    }
}

register_client!(
    GroupConversationRequest,
    co => "conversations",
);

impl<'a> GroupConversationRequest<'a> {
    get!( list, Collection<serde_json::Value> => "groups/{{RID}}/{{co}}" );
    get!( | list_threads, Collection<serde_json::Value> => "groups/{{RID}}/{{co}}/{{id}}/threads" );
    get!( list_accepted_senders, Collection<serde_json::Value> => "groups/{{RID}}/acceptedSenders" );
    get!( | get, serde_json::Value => "groups/{{RID}}/{{co}}/{{id}}" );
    post!( [ create, serde_json::Value => "groups/{{RID}}/{{co}}" ] );
    post!( [ | create_thread, serde_json::Value => "groups/{{RID}}/{{co}}/{{id}}/threads" ] );
    post!( [ create_accepted_sender, GraphResponse<Content> => "groups/{{RID}}/acceptedSenders/$ref" ] );
    delete!( | delete, GraphResponse<Content> => "groups/{{RID}}/{{co}}/{{id}}" );

    pub fn thread_posts(&'a self) -> GroupThreadPostRequest<'a> {
        GroupThreadPostRequest::new(self.client)
    }

    pub fn conversation_posts(&'a self) -> GroupConversationPostRequest<'a> {
        GroupConversationPostRequest::new(self.client)
    }
}

register_client!(GroupThreadPostRequest,);

impl<'a> GroupThreadPostRequest<'a> {
    get!( | list, Collection<serde_json::Value> => "groups/{{RID}}/threads/{{id}}/posts" );
    get!( || get, serde_json::Value => "groups/{{RID}}/threads/{{id}}/posts/{{id2}}" );
    post!( [ || reply, GraphResponse<Content> => "groups/{{RID}}/threads/{{id}}/posts/{{id2}}/reply" ] );
    post!( [ || forward, GraphResponse<Content> => "groups/{{RID}}/threads/{{id}}/posts/{{id2}}/forward" ] );

    pub fn attachments(&'a self) -> ThreadPostAttachmentRequest<'a> {
        render_path!(self.client, "groups/{{RID}}");
        ThreadPostAttachmentRequest::new(self.client)
    }
}

register_client!(GroupConversationPostRequest,);

impl<'a> GroupConversationPostRequest<'a> {
    get!( || list, Collection<serde_json::Value> => "groups/{{RID}}/conversations/{{id}}/threads/{{id2}}/posts" );
    get!( ||| get, serde_json::Value => "groups/{{RID}}/conversations/{{id}}/threads/{{id2}}/posts/{{id3}}" );
    post!( [ ||| reply, GraphResponse<Content> => "groups/{{RID}}/conversations/{{id}}/threads/{{id2}}/posts/{{id3}}/reply" ] );
    post!( [ ||| forward, GraphResponse<Content> => "groups/{{RID}}/conversations/{{id}}/threads/{{id2}}/posts/{{id3}}/forward" ] );

    pub fn attachments(&'a self) -> ThreadConvoPostAttachmentRequest<'a> {
        render_path!(self.client, "groups/{{RID}}");
        ThreadConvoPostAttachmentRequest::new(self.client)
    }
}

impl<'a> IdentUsers<'a> {
    get!( get, serde_json::Value => "users/{{RID}}" );
    get!( settings, serde_json::Value => "users/{{RID}}/settings" );
    get!( list, Collection<serde_json::Value> => "users" );
    get!( list_events, Collection<serde_json::Value> => "users/{{RID}}/events" );
    get!( delta, DeltaRequest<Collection<serde_json::Value>> => "users" );
    get!( | list_joined_group_photos, Collection<serde_json::Value> => "users/{{RID}}/joinedGroups/{{id}}/photos" );
    post!( [ create, serde_json::Value => "users" ] );
    patch!( [ update, GraphResponse<Content> => "users/{{RID}}" ] );
    patch!( [ update_settings, serde_json::Value => "users/{{RID}}/settings" ] );
    delete!( delete, GraphResponse<Content> => "users/{{RID}}" );
}

register_ident_client!(
    GroupLifecyclePolicyRequest,
    glp => "groupLifecyclePolicies",
    ()
);

impl<'a> GroupLifecyclePolicyRequest<'a> {
    get!( list, Collection<serde_json::Value> => "{{glp}}" );
    get!( get, Collection<serde_json::Value> => "{{glp}}/{{RID}}" );
    post!( [ create, serde_json::Value => "{{glp}}" ] );
    post!( [ add_group, BoolResponse => "{{glp}}/{{RID}}/addGroup" ] );
    post!( [ remove_group, BoolResponse =>  "{{glp}}/{{RID}}/removeGroup" ] );
    patch!( [ update, serde_json::Value => "{{glp}}/{{RID}}" ] );
    patch!( delete, GraphResponse<Content> => "{{glp}}/{{RID}}" );
}
