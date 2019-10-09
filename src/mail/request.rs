use crate::client::Graph;
use crate::http::{GraphResponse, IntoResponse};
use crate::types::{collection::Collection, content::Content};
use graph_rs_types::complextypes::MailTips;
use graph_rs_types::entitytypes::{Attachment, MailFolder, Message, MessageRule};
use handlebars::*;
use reqwest::Method;
use std::marker::PhantomData;

register_client!(
    MailRequest,
    mm => "messages",
    mf => "mailFolders",
);

impl<'a, I> MailRequest<'a, I> {
    get!( mail_tips, Collection<MailTips> => "getMailTips" );

    pub fn messages(&'a self) -> MessageRequest<'a, I> {
        MessageRequest::new(self.client)
    }

    pub fn mail_folder(&'a self) -> MailFolderRequest<'a, I> {
        MailFolderRequest::new(self.client)
    }
}

register_client!(MessageRequest,);

impl<'a, I> MessageRequest<'a, I> {
    get!( list, Collection<Message> => "{{mm}}" );
    get!( | get, Collection<Message> => "{{mm}}/{{id}}" );
    get!( list_attachments, Collection<Attachment> => "{{mm}}/{{id}}/attachments" );
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
    post!( [ | add_attachment, Attachment => "{{mm}}/{{id}}/attachments" ] );
    patch!( [ | update, Message => "{{mm}}/{{id}}" ] );
    delete!( | delete, GraphResponse<Content> => "{{mm}}/{{id}}" );
}

register_client!(MailFolderRequest,);

impl<'a, I> MailFolderRequest<'a, I> {
    get!( list, Collection<MailFolder> => "{{mf}}" );
    get!( | get, MailFolder => "{{mf}}/{{id}}" );
    get!( list_rules, Collection<MessageRule> => "{{mf}}/inbox/messageRules" );
    get!( [ create_rule, MessageRule => "{{mf}}/inbox/messageRules" ] );
    post!( [ | copy, MailFolder => "{{mf}}/{{id}}/copy" ] );
    post!( [ create, MailFolder => "{{mf}}" ] );
    patch!( [ | update, MailFolder => "{{mf}}/{{id}}" ] );
    delete!( | delete, GraphResponse<Content> => "{{mf}}/{{id}}" );

    pub fn messages(&'a self) -> MailFolderMessageRequest<'a, I> {
        MailFolderMessageRequest::new(self.client)
    }
}

register_client!(MailFolderMessageRequest,);

impl<'a, I> MailFolderMessageRequest<'a, I> {
    get!( | list, Collection<Message> => "{{mf}}/{{id}}/messages" );
    get!( || get, Collection<Message> => "{{mf}}/{{id}}/{{mm}}/{{id2}}" );
    get!( || list_attachments, Collection<Attachment> => "{{mf}}/{{id}}/{{mm}}/{{id2}}/attachments" );
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
}
