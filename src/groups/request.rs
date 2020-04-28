use crate::attachments::{ThreadConvoPostAttachmentRequest, ThreadPostAttachmentRequest};
use crate::client::Graph;
use crate::http::{GraphResponse, IntoResponse};
use crate::types::{collection::Collection, content::Content};
use handlebars::*;
use reqwest::Method;

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
