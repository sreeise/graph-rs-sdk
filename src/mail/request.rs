use crate::client::Graph;
use graph_http::types::{Collection, Content, DeltaPhantom};
use graph_http::{GraphResponse, IntoResponse};
use handlebars::*;
use reqwest::Method;

register_client!(
    MailRequest,
    mm => "messages",
    mf => "mailFolders",
    mfmr => "mailFolders/inbox/messageRules",
    ico => "inferenceClassification/overrides",
    olc => "outlook/masterCategories",
);

impl<'a, Client> MailRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( mail_tips, Collection<serde_json::Value> => "getMailTips" );

    pub fn messages(&'a self) -> MessageRequest<'a, Client> {
        MessageRequest::new(self.client)
    }

    pub fn mail_folder(&'a self) -> MailFolderRequest<'a, Client> {
        MailFolderRequest::new(self.client)
    }

    pub fn focused_inbox(&'a self) -> FocusedInboxRequest<'a, Client> {
        FocusedInboxRequest::new(self.client)
    }

    pub fn outlook_category(&'a self) -> OutlookCategoryRequest<'a, Client> {
        OutlookCategoryRequest::new(self.client)
    }
}

register_client!(MessageRequest,);

impl<'a, Client> MessageRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( list, Collection<serde_json::Value> => "{{mm}}" );
    get!( | get, serde_json::Value => "{{mm}}/{{id}}" );
    get!( content, GraphResponse<Content> => "{{mm}}/{{id}}/$value" );
    post!( | create_reply, serde_json::Value => "{{mm}}/{{id}}/createReply" );
    post!( | create_reply_all, serde_json::Value => "{{mm}}/{{id}}/createReplyAll" );
    post!( | create_forward, serde_json::Value => "{{mm}}/{{id}}/createForward" );
    post!( [ | forward, GraphResponse<Content> => "{{mm}}/{{id}}/forward" ] );
    post!( | send_message, serde_json::Value => "{{mm}}/{{id}}/send" );
    post!( [ create, serde_json::Value => "{{mm}}" ] );
    post!( [ send_mail, GraphResponse<Content> => "sendMail" ] );
    post!( [ | copy, serde_json::Value => "{{mm}}/{{id}}/copy" ] );
    post!( [ | move_message, serde_json::Value => "{{mm}}/{{id}}/move" ] );
    post!( [ | reply, GraphResponse<Content> => "{{mm}}/{{id}}/reply" ] );
    post!( [ | reply_all, GraphResponse<Content> => "{{mm}}/{{id}}/replyAll" ] );
    patch!( [ | update, serde_json::Value => "{{mm}}/{{id}}" ] );
    delete!( | delete, GraphResponse<Content> => "{{mm}}/{{id}}" );
}

register_client!(MailFolderRequest,);

impl<'a, Client> MailFolderRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( list, Collection<serde_json::Value> => "{{mf}}" );
    get!( | list_child_folders, Collection<serde_json::Value> => "{{mf}}/{{id}}/childFolders" );
    get!( | get, serde_json::Value => "{{mf}}/{{id}}" );
    get!( delta, DeltaPhantom<Collection<serde_json::Value>> => "{{mf}}/delta" );
    get!( archive, serde_json::Value => "{{mf}}/archive" );
    get!( inbox, serde_json::Value => "{{mf}}/inbox" );
    get!( clutter, serde_json::Value => "{{mf}}/clutter" );
    get!( conflicts, serde_json::Value => "{{mf}}/conflicts" );
    get!( conversation_history, serde_json::Value => "{{mf}}/conversationhistory" );
    get!( deleted_items, serde_json::Value => "{{mf}}/deleteditems" );
    get!( drafts, serde_json::Value => "{{mf}}/drafts" );
    get!( junk_email, serde_json::Value => "{{mf}}/junkemail" );
    get!( local_failures, serde_json::Value => "{{mf}}/localfailures" );
    get!( msg_folder_root, serde_json::Value => "{{mf}}/msgfolderroot" );
    get!( outbox, serde_json::Value => "{{mf}}/outbox" );
    get!( recoverable_items_deletions, serde_json::Value => "{{mf}}/recoverableitemsdeletions" );
    get!( scheduled, serde_json::Value => "{{mf}}/scheduled" );
    get!( search_folders, serde_json::Value => "{{mf}}/searchfolders" );
    get!( send_items, serde_json::Value => "{{mf}}/sentitems" );
    get!( server_failures, serde_json::Value => "{{mf}}/serverfailures" );
    get!( sync_issues, serde_json::Value => "{{mf}}/syncissues" );
    post!( [ | copy, serde_json::Value => "{{mf}}/{{id}}/copy" ] );
    post!( [ create,serde_json::Value => "{{mf}}" ] );
    post!( [ create_child_folder, serde_json::Value => "{{mf}}/childFolders" ] );
    post!( [ | move_mail_folder, serde_json::Value => "{{mf}}/{{id}}/move" ] );
    patch!( [ | update, serde_json::Value => "{{mf}}/{{id}}" ] );
    delete!( | delete, GraphResponse<Content> => "{{mf}}/{{id}}" );

    pub fn messages(&'a self) -> MailFolderMessageRequest<'a, Client> {
        MailFolderMessageRequest::new(self.client)
    }

    pub fn rules(&'a self) -> MailRuleRequest<'a, Client> {
        MailRuleRequest::new(self.client)
    }
}

register_client!(MailFolderMessageRequest,);

impl<'a, Client> MailFolderMessageRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( | list, Collection<serde_json::Value> => "{{mf}}/{{id}}/messages" );
    get!( || get, Collection<serde_json::Value> => "{{mf}}/{{id}}/{{mm}}/{{id2}}" );
    get!( || list_attachments, Collection<serde_json::Value> => "{{mf}}/{{id}}/{{mm}}/{{id2}}/attachments" );
    get!( list_archive, Collection<serde_json::Value> => "{{mf}}/archive/messages" );
    get!( list_inbox, Collection<serde_json::Value> => "{{mf}}/inbox/messages" );
    get!( list_clutter, Collection<serde_json::Value> => "{{mf}}/clutter/messages" );
    get!( list_conflicts, Collection<serde_json::Value> => "{{mf}}/conflicts/messages" );
    get!( list_conversation_history, Collection<serde_json::Value> => "{{mf}}/conversationhistory/messages" );
    get!( list_deleted_items, Collection<serde_json::Value> => "{{mf}}/deleteditems/messages" );
    get!( list_drafts, Collection<serde_json::Value> => "{{mf}}/drafts/messages" );
    get!( list_junk_email, Collection<serde_json::Value> => "{{mf}}/junkemail/messages" );
    get!( list_local_failures, Collection<serde_json::Value> => "{{mf}}/localfailures/messages" );
    get!( list_msg_folder_root, Collection<serde_json::Value> => "{{mf}}/msgfolderroot/messages" );
    get!( list_outbox, Collection<serde_json::Value> => "{{mf}}/outbox/messages" );
    get!( list_recoverable_items_deletions, Collection<serde_json::Value> => "{{mf}}/recoverableitemsdeletions/messages" );
    get!( list_scheduled, Collection<serde_json::Value> => "{{mf}}/scheduled/messages" );
    get!( list_search_folders, Collection<serde_json::Value> => "{{mf}}/searchfolders/messages" );
    get!( list_send_items, Collection<serde_json::Value> => "{{mf}}/sentitems/messages" );
    get!( list_server_failures, Collection<serde_json::Value> => "{{mf}}/serverfailures/messages" );
    get!( list_sync_issues, Collection<serde_json::Value> => "{{mf}}/syncissues/messages" );
    get!( | archive, serde_json::Value => "{{mf}}/archive/messages/{{id}}" );
    get!( | inbox, serde_json::Value => "{{mf}}/inbox/messages/{{id}}" );
    get!( | clutter, serde_json::Value => "{{mf}}/clutter/messages/{{id}}" );
    get!( | conflicts, serde_json::Value => "{{mf}}/conflicts/messages/{{id}}" );
    get!( | conversation_history, serde_json::Value => "{{mf}}/conversationhistory/messages/{{id}}" );
    get!( | deleted_items, serde_json::Value => "{{mf}}/deleteditems/messages/{{id}}" );
    get!( | drafts, serde_json::Value => "{{mf}}/drafts/messages/{{id}}" );
    get!( | junk_email, serde_json::Value => "{{mf}}/junkemail/messages/{{id}}" );
    get!( | local_failures, serde_json::Value => "{{mf}}/localfailures/messages/{{id}}" );
    get!( | msg_folder_root, serde_json::Value => "{{mf}}/msgfolderroot/messages/{{id}}" );
    get!( | outbox, serde_json::Value => "{{mf}}/outbox/messages" );
    get!( | recoverable_items_deletions, serde_json::Value => "{{mf}}/recoverableitemsdeletions/messages/{{id}}" );
    get!( | scheduled, serde_json::Value => "{{mf}}/scheduled/messages/{{id}}" );
    get!( | search_folders, serde_json::Value => "{{mf}}/searchfolders/messages/{{id}}" );
    get!( | send_items, serde_json::Value => "{{mf}}/sentitems/messages/{{id}}" );
    get!( | server_failures, serde_json::Value => "{{mf}}/serverfailures/messages/{{id}}" );
    get!( | sync_issues, serde_json::Value => "{{mf}}/syncissues/messages/{{id}}" );
    post!( [ || reply, GraphResponse<Content> => "{{mf}}/{{id}}/{{mm}}/{{id2}}/reply" ] );
    post!( [ || reply_all, GraphResponse<Content> => "{{mf}}/{{id}}/{{mm}}/{{id2}}/replyAll" ] );
    post!( [ || copy, serde_json::Value => "{{mf}}/{{id}}/{{mm}}/{{id2}}/copy" ] );
    post!( [ || move_message, serde_json::Value => "{{mf}}/{{id}}/{{mm}}/{{id2}}/move" ] );
    post!( [ || forward, GraphResponse<Content> => "{{mf}}/{{id}}/{{mm}}/{{id2}}/forward" ] );
    post!( || create_forward, serde_json::Value => "{{mf}}/{{id}}/{{mm}}/{{id2}}/createForward" );
    post!( [ | create, serde_json::Value => "{{mf}}/{{id}}/{{mm}}" ] );
    post!( || create_reply, serde_json::Value => "{{mf}}/{{id}}/{{mm}}/{{id2}}/createReply" );
    post!( || create_reply_all, serde_json::Value => "{{mf}}/{{id}}/{{mm}}/{{id2}}/createReplyAll" );
    post!( [ send_mail, GraphResponse<Content> => "sendMail" ] );
    post!( [ || add_attachment, serde_json::Value => "{{mf}}/{{id}}/{{mm}}/{{id2}}/attachments" ] );
    patch!( [ || update, serde_json::Value => "{{mf}}/{{id}}/{{mm}}/{{id2}}" ] );
    delete!( || delete, GraphResponse<Content> => "{{mf}}/{{id}}/{{mm}}/{{id2}}" );
}

register_client!(MailRuleRequest,);

impl<'a, Client> MailRuleRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( list, Collection<serde_json::Value> => "{{mfmr}}" );
    get!( | get, serde_json::Value => "{{mfmr}}/{{id}}" );
    post!( [ create, serde_json::Value => "{{mfmr}}" ] );
    patch!( [ | update,serde_json::Value => "{{mfmr}}/{{id}}" ] );
    delete!( | delete, GraphResponse<Content> => "{{mfmr}}/{{id}}" );
}

register_client!(FocusedInboxRequest,);

impl<'a, Client> FocusedInboxRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( list_overrides, Collection<serde_json::Value> => "{{ico}}" );
    patch!( [ create_override, serde_json::Value=> "{{ico}}" ] );
    patch!( [ | update_override, serde_json::Value => "{{ico}}/{{id}}" ] );
    delete!( | delete_override, GraphResponse<Content> => "{{ico}}/{{id}}"  );
}

register_client!(OutlookCategoryRequest,);

impl<'a, Client> OutlookCategoryRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( list, Collection<serde_json::Value> => "{{olc}}" );
    get!( | get, serde_json::Value => "{{olc}}/{{id}}" );
    post!( [ create, serde_json::Value  => "{{olc}}" ] );
    patch!( [ | update, serde_json::Value  => "{{olc}}/{{id}}" ] );
    delete!( | delete, GraphResponse<Content> => "{{olc}}/{{id}}" );
}
