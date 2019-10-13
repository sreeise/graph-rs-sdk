use crate::attachments::{MailFolderMessageAttachmentRequest, MailMessageAttachmentRequest};
use crate::client::Graph;
use crate::http::{GraphResponse, IntoResponse};
use crate::types::{collection::Collection, content::Content, delta::DeltaRequest};
use graph_rs_types::complextypes::MailTips;
use graph_rs_types::entitytypes::{
    Attachment, InferenceClassificationOverride, MailFolder, Message, MessageRule, OutlookCategory,
};
use handlebars::*;
use reqwest::Method;
use std::marker::PhantomData;

register_client!(
    MailRequest,
    mm => "messages",
    mf => "mailFolders",
    mfmr => "mailFolders/inbox/messageRules",
    ico => "inferenceClassification/overrides",
    olc => "outlook/masterCategories",
);

impl<'a, I> MailRequest<'a, I> {
    get!( mail_tips, Collection<MailTips> => "getMailTips" );

    pub fn messages(&'a self) -> MessageRequest<'a, I> {
        MessageRequest::new(self.client)
    }

    pub fn mail_folder(&'a self) -> MailFolderRequest<'a, I> {
        MailFolderRequest::new(self.client)
    }

    pub fn focused_inbox(&'a self) -> FocusedInboxRequest<'a, I> {
        FocusedInboxRequest::new(self.client)
    }

    pub fn outlook_category(&'a self) -> OutlookCategoryRequest<'a, I> {
        OutlookCategoryRequest::new(self.client)
    }
}

register_client!(MessageRequest,);

impl<'a, I> MessageRequest<'a, I> {
    get!( list, Collection<Message> => "{{mm}}" );
    get!( | get, Message => "{{mm}}/{{id}}" );
    get!( content, GraphResponse<Content> => "{{mm}}/{{id}}/$value" );
    post!( | create_reply, Message => "{{mm}}/{{id}}/createReply" );
    post!( | create_reply_all, Message => "{{mm}}/{{id}}/createReplyAll" );
    post!( | create_forward, Message => "{{mm}}/{{id}}/createForward" );
    post!( [ | forward, GraphResponse<Content> => "{{mm}}/{{id}}/forward" ] );
    post!( | send_message, Message => "{{mm}}/{{id}}/send" );
    post!( [ create, Message => "{{mm}}" ] );
    post!( [ send_mail, GraphResponse<Content> => "sendMail" ] );
    post!( [ | copy, Message => "{{mm}}/{{id}}/copy" ] );
    post!( [ | move_message, Message => "{{mm}}/{{id}}/move" ] );
    post!( [ | reply, GraphResponse<Content> => "{{mm}}/{{id}}/reply" ] );
    post!( [ | reply_all, GraphResponse<Content> => "{{mm}}/{{id}}/replyAll" ] );
    patch!( [ | update, Message => "{{mm}}/{{id}}" ] );
    delete!( | delete, GraphResponse<Content> => "{{mm}}/{{id}}" );

    pub fn attachments(&'a self) -> MailMessageAttachmentRequest<'a, I> {
        MailMessageAttachmentRequest::new(self.client)
    }
}

register_client!(MailFolderRequest,);

impl<'a, I> MailFolderRequest<'a, I> {
    get!( list, Collection<MailFolder> => "{{mf}}" );
    get!( | list_child_folders, Collection<MailFolder> => "{{mf}}/{{id}}/childFolders" );
    get!( | get, MailFolder => "{{mf}}/{{id}}" );
    get!( delta, DeltaRequest<Collection<MailFolder>> => "{{mf}}/delta" );
    get!( archive, MailFolder => "{{mf}}/archive" );
    get!( inbox, MailFolder => "{{mf}}/inbox" );
    get!( clutter, MailFolder => "{{mf}}/clutter" );
    get!( conflicts, MailFolder => "{{mf}}/conflicts" );
    get!( conversation_history, MailFolder => "{{mf}}/conversationhistory" );
    get!( deleted_items, MailFolder => "{{mf}}/deleteditems" );
    get!( drafts, MailFolder => "{{mf}}/drafts" );
    get!( junk_email, MailFolder => "{{mf}}/junkemail" );
    get!( local_failures, MailFolder => "{{mf}}/localfailures" );
    get!( msg_folder_root, MailFolder => "{{mf}}/msgfolderroot" );
    get!( outbox, MailFolder => "{{mf}}/outbox" );
    get!( recoverable_items_deletions, MailFolder => "{{mf}}/recoverableitemsdeletions" );
    get!( scheduled, MailFolder => "{{mf}}/scheduled" );
    get!( search_folders, MailFolder => "{{mf}}/searchfolders" );
    get!( send_items, MailFolder => "{{mf}}/sentitems" );
    get!( server_failures, MailFolder => "{{mf}}/serverfailures" );
    get!( sync_issues, MailFolder => "{{mf}}/syncissues" );
    post!( [ | copy, MailFolder => "{{mf}}/{{id}}/copy" ] );
    post!( [ create, MailFolder => "{{mf}}" ] );
    post!( [ create_child_folder, MailFolder => "{{mf}}/childFolders" ] );
    post!( [ | move_mail_folder, MailFolder => "{{mf}}/{{id}}/move" ] );
    patch!( [ | update, MailFolder => "{{mf}}/{{id}}" ] );
    delete!( | delete, GraphResponse<Content> => "{{mf}}/{{id}}" );

    pub fn messages(&'a self) -> MailFolderMessageRequest<'a, I> {
        MailFolderMessageRequest::new(self.client)
    }

    pub fn rules(&'a self) -> MailRuleRequest<'a, I> {
        MailRuleRequest::new(self.client)
    }

    pub fn attachments(&'a self) -> MailFolderMessageAttachmentRequest<'a, I> {
        MailFolderMessageAttachmentRequest::new(self.client)
    }
}

register_client!(MailFolderMessageRequest,);

impl<'a, I> MailFolderMessageRequest<'a, I> {
    get!( | list, Collection<Message> => "{{mf}}/{{id}}/messages" );
    get!( || get, Collection<Message> => "{{mf}}/{{id}}/{{mm}}/{{id2}}" );
    get!( || list_attachments, Collection<Attachment> => "{{mf}}/{{id}}/{{mm}}/{{id2}}/attachments" );
    get!( list_archive, Collection<Message> => "{{mf}}/archive/messages" );
    get!( list_inbox, Collection<Message> => "{{mf}}/inbox/messages" );
    get!( list_clutter, Collection<Message> => "{{mf}}/clutter/messages" );
    get!( list_conflicts, Collection<Message> => "{{mf}}/conflicts/messages" );
    get!( list_conversation_history, Collection<Message> => "{{mf}}/conversationhistory/messages" );
    get!( list_deleted_items, Collection<Message> => "{{mf}}/deleteditems/messages" );
    get!( list_drafts, Collection<Message> => "{{mf}}/drafts/messages" );
    get!( list_junk_email, Collection<Message> => "{{mf}}/junkemail/messages" );
    get!( list_local_failures, Collection<Message> => "{{mf}}/localfailures/messages" );
    get!( list_msg_folder_root, Collection<Message> => "{{mf}}/msgfolderroot/messages" );
    get!( list_outbox, Collection<Message> => "{{mf}}/outbox/messages" );
    get!( list_recoverable_items_deletions, Collection<Message> => "{{mf}}/recoverableitemsdeletions/messages" );
    get!( list_scheduled, Collection<Message> => "{{mf}}/scheduled/messages" );
    get!( list_search_folders, Collection<Message> => "{{mf}}/searchfolders/messages" );
    get!( list_send_items, Collection<Message> => "{{mf}}/sentitems/messages" );
    get!( list_server_failures, Collection<Message> => "{{mf}}/serverfailures/messages" );
    get!( list_sync_issues, Collection<Message> => "{{mf}}/syncissues/messages" );
    get!( | archive, Message => "{{mf}}/archive/messages/{{id}}" );
    get!( | inbox, Message => "{{mf}}/inbox/messages/{{id}}" );
    get!( | clutter, Message => "{{mf}}/clutter/messages/{{id}}" );
    get!( | conflicts, Message => "{{mf}}/conflicts/messages/{{id}}" );
    get!( | conversation_history, Message => "{{mf}}/conversationhistory/messages/{{id}}" );
    get!( | deleted_items, Message => "{{mf}}/deleteditems/messages/{{id}}" );
    get!( | drafts, Message => "{{mf}}/drafts/messages/{{id}}" );
    get!( | junk_email, Message => "{{mf}}/junkemail/messages/{{id}}" );
    get!( | local_failures, Message => "{{mf}}/localfailures/messages/{{id}}" );
    get!( | msg_folder_root, Message => "{{mf}}/msgfolderroot/messages/{{id}}" );
    get!( | outbox, Message => "{{mf}}/outbox/messages" );
    get!( | recoverable_items_deletions, Message => "{{mf}}/recoverableitemsdeletions/messages/{{id}}" );
    get!( | scheduled, Message => "{{mf}}/scheduled/messages/{{id}}" );
    get!( | search_folders, Message => "{{mf}}/searchfolders/messages/{{id}}" );
    get!( | send_items, Message => "{{mf}}/sentitems/messages/{{id}}" );
    get!( | server_failures, Message => "{{mf}}/serverfailures/messages/{{id}}" );
    get!( | sync_issues, Message => "{{mf}}/syncissues/messages/{{id}}" );
    post!( [ || reply, GraphResponse<Content> => "{{mf}}/{{id}}/{{mm}}/{{id2}}/reply" ] );
    post!( [ || reply_all, GraphResponse<Content> => "{{mf}}/{{id}}/{{mm}}/{{id2}}/replyAll" ] );
    post!( [ || copy, Message => "{{mf}}/{{id}}/{{mm}}/{{id2}}/copy" ] );
    post!( [ || move_message, Message => "{{mf}}/{{id}}/{{mm}}/{{id2}}/move" ] );
    post!( [ || forward, GraphResponse<Content> => "{{mf}}/{{id}}/{{mm}}/{{id2}}/forward" ] );
    post!( || create_forward, Message => "{{mf}}/{{id}}/{{mm}}/{{id2}}/createForward" );
    post!( [ | create, Message => "{{mf}}/{{id}}/{{mm}}" ] );
    post!( || create_reply, Message => "{{mf}}/{{id}}/{{mm}}/{{id2}}/createReply" );
    post!( || create_reply_all, Message => "{{mf}}/{{id}}/{{mm}}/{{id2}}/createReplyAll" );
    post!( [ send_mail, GraphResponse<Content> => "sendMail" ] );
    post!( [ || add_attachment, Attachment => "{{mf}}/{{id}}/{{mm}}/{{id2}}/attachments" ] );
    patch!( [ || update, Message => "{{mf}}/{{id}}/{{mm}}/{{id2}}" ] );
    delete!( || delete, GraphResponse<Content> => "{{mf}}/{{id}}/{{mm}}/{{id2}}" );

    pub fn attachments(&'a self) -> MailFolderMessageAttachmentRequest<'a, I> {
        MailFolderMessageAttachmentRequest::new(self.client)
    }
}

register_client!(MailRuleRequest,);

impl<'a, I> MailRuleRequest<'a, I> {
    get!( list, Collection<MessageRule> => "{{mfmr}}" );
    get!( | get, MessageRule => "{{mfmr}}/{{id}}" );
    post!( [ create, MessageRule => "{{mfmr}}" ] );
    patch!( [ | update, MessageRule => "{{mfmr}}/{{id}}" ] );
    delete!( | delete, GraphResponse<Content> => "{{mfmr}}/{{id}}" );
}

register_client!(FocusedInboxRequest,);

impl<'a, I> FocusedInboxRequest<'a, I> {
    get!( list_overrides, Collection<InferenceClassificationOverride> => "{{ico}}" );
    patch!( [ create_override, InferenceClassificationOverride => "{{ico}}" ] );
    patch!( [ | update_override, InferenceClassificationOverride => "{{ico}}/{{id}}" ] );
    delete!( | delete_override, GraphResponse<Content> => "{{ico}}/{{id}}"  );
}

register_client!(OutlookCategoryRequest,);

impl<'a, I> OutlookCategoryRequest<'a, I> {
    get!( list, Collection<OutlookCategory> => "{{olc}}" );
    get!( | get, OutlookCategory => "{{olc}}/{{id}}" );
    post!( [ create, OutlookCategory  => "{{olc}}" ] );
    patch!( [ | update, OutlookCategory  => "{{olc}}/{{id}}" ] );
    delete!( | delete, GraphResponse<Content> => "{{olc}}/{{id}}" );
}
