use crate::client::Graph;
use crate::http::GraphResponse;
use crate::http::ResponseClient;
use crate::types::collection::Collection;
use crate::url::{FormatOrd, UrlOrdering};
use graph_rs_types::entitytypes::Message;
use reqwest::Method;
use serde_json::json;
use std::marker::PhantomData;

fn message_vec() -> Vec<FormatOrd> {
    vec![FormatOrd::Insert(UrlOrdering::ItemPath("messages".into()))]
}

fn message_vec_extend(other: Vec<FormatOrd>) -> Vec<FormatOrd> {
    let mut v = message_vec();
    v.extend(other);
    v
}

client_struct!(MailRequest);

impl<'a, I> MailRequest<'a, I> {
    pub fn messages(&'a self) -> MessageRequest<'a, I> {
        MessageRequest::new(self.client)
    }

    pub fn mail_folder(&'a self) -> MailFolderRequest<'a, I> {
        MailFolderRequest::new(self.client)
    }
}

client_struct!(MessageRequest);

impl<'a, I> MessageRequest<'a, I> {
    get!(list, Collection<Message>, message_vec(), true);
    get!(get, Message, message_vec(), false);
    delete!(delete, GraphResponse<()>, message_vec(), false);
    post!(
        create_reply,
        Message,
        message_vec_extend(vec![FormatOrd::Insert(UrlOrdering::Last(
            "createReply".into()
        ))]),
        false
    );
    post!(
        create_reply_all,
        Message,
        message_vec_extend(vec![FormatOrd::Insert(UrlOrdering::Last(
            "createReplyAll".into()
        ))]),
        false
    );
    post!(
        create_forward,
        Message,
        message_vec_extend(vec![FormatOrd::Insert(UrlOrdering::Last(
            "createForward".into()
        ))]),
        false
    );
    post!(
        send_message,
        GraphResponse<()>,
        message_vec_extend(vec![FormatOrd::Insert(UrlOrdering::Last("send".into()))]),
        false
    );
    post!(update, Message, message_vec(), false, ());
    post!(create, Message, message_vec(), true, ());
    post!(
        send_mail,
        GraphResponse<()>,
        vec![FormatOrd::Insert(UrlOrdering::Last("sendMail".into()))],
        true,
        ()
    );
    post!(
        copy,
        &str,
        "destination_id",
        Message,
        message_vec_extend(vec![FormatOrd::Insert(UrlOrdering::Last("copy".into()))]),
        false
    );
    post!(
        move_message,
        &str,
        "destination_id",
        Message,
        message_vec_extend(vec![FormatOrd::Insert(UrlOrdering::Last("move".into()))]),
        false
    );
    post!(
        reply,
        &str,
        "comment",
        GraphResponse::<()>,
        message_vec_extend(vec![FormatOrd::Insert(UrlOrdering::Last("reply".into()))]),
        false
    );
    post!(
        reply_all,
        &str,
        "comment",
        GraphResponse::<()>,
        message_vec_extend(vec![FormatOrd::Insert(UrlOrdering::Last(
            "replyAll".into()
        ))]),
        false
    );

    pub fn forward<T: serde::Serialize>(
        &'a self,
        comment: &str,
        to_recipients: &[T],
    ) -> ResponseClient<'a, I, GraphResponse<()>> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(UrlOrdering::ItemPath("messages".into()))
            .insert(UrlOrdering::Last("forward".into()))
            .set_body(
                serde_json::to_string_pretty(
                    &json!({ "comment": comment, "toRecipients": to_recipients }),
                )
                .unwrap(),
            );
        ResponseClient::new(self.client)
    }
}

fn format_mail_folder_message(s: &str) -> UrlOrdering {
    UrlOrdering::ItemPath(format!("mailFolders/{}/messages", s))
}

fn ord_fn() -> Box<dyn Fn(String) -> Vec<FormatOrd>> {
    Box::new(|s: String| {
        vec![FormatOrd::Insert(UrlOrdering::ItemPath(format!(
            "mailFolders/{}/messages",
            s
        )))]
    })
}

fn ord_fn_last(last: String) -> Box<dyn Fn(String) -> Vec<FormatOrd>> {
    Box::new(move |s: String| {
        vec![FormatOrd::Insert(UrlOrdering::ItemPath(format!(
            "mailFolders/{}/{}",
            s, last
        )))]
    })
}

client_struct!(MailFolderRequest);

impl<'a, I> MailFolderRequest<'a, I> {
    request_method_fn!(
        list,
        Collection<Message>,
        vec![],
        ord_fn(),
        Method::GET,
        true
    );
    request_method_fn!(get, Message, vec![], ord_fn(), Method::GET, false);
    request_method_fn!(
        delete,
        GraphResponse<()>,
        vec![],
        ord_fn(),
        Method::DELETE,
        false
    );
    request_method_fn!(
        create_reply,
        Message,
        vec![FormatOrd::Insert(UrlOrdering::Last("createReply".into()))],
        ord_fn(),
        Method::POST,
        false
    );
    request_method_fn!(
        create_reply_all,
        Message,
        vec![FormatOrd::Insert(UrlOrdering::Last(
            "createReplyAll".into()
        ))],
        ord_fn(),
        Method::POST,
        false
    );
    request_method_fn!(
        create_forward,
        Message,
        vec![FormatOrd::Insert(UrlOrdering::Last("createForward".into()))],
        ord_fn(),
        Method::POST,
        false
    );
    request_method_fn!(
        send_message,
        GraphResponse<()>,
        vec![FormatOrd::Insert(UrlOrdering::Last("send".into()))],
        ord_fn(),
        Method::POST,
        false
    );
    request_method_fn!(update, Message, vec![], ord_fn(), Method::PATCH, false, ());
    request_method_fn!(create, Message, vec![], ord_fn(), Method::POST, true, ());
    request_method_fn!(
        send_mail,
        GraphResponse<()>,
        vec![],
        ord_fn_last("sendMail".into()),
        Method::POST,
        true,
        ()
    );

    pub fn copy(
        &'a self,
        mail_folder_id: &str,
        destination_id: &str,
    ) -> ResponseClient<'a, I, Message> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(format_mail_folder_message(mail_folder_id))
            .insert(UrlOrdering::Last("copy".into()))
            .set_body(
                serde_json::to_string_pretty(&json!({ "destination_id": destination_id })).unwrap(),
            );
        ResponseClient::new(self.client)
    }

    pub fn move_message(
        &'a self,
        mail_folder_id: &str,
        destination_id: &str,
    ) -> ResponseClient<'a, I, Message> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(format_mail_folder_message(mail_folder_id))
            .insert(UrlOrdering::Last("move".into()))
            .set_body(
                serde_json::to_string_pretty(&json!({ "destination_id": destination_id })).unwrap(),
            );
        ResponseClient::new(self.client)
    }

    pub fn reply(
        &'a self,
        mail_folder_id: &str,
        comment: &str,
    ) -> ResponseClient<'a, I, GraphResponse<()>> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(format_mail_folder_message(mail_folder_id))
            .insert(UrlOrdering::Last("reply".into()))
            .set_body(serde_json::to_string_pretty(&json!({ "comment": comment })).unwrap());
        ResponseClient::new(self.client)
    }

    pub fn reply_all(
        &'a self,
        mail_folder_id: &str,
        comment: &str,
    ) -> ResponseClient<'a, I, GraphResponse<()>> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(format_mail_folder_message(mail_folder_id))
            .insert(UrlOrdering::Last("replyAll".into()))
            .set_body(serde_json::to_string_pretty(&json!({ "comment": comment })).unwrap());
        ResponseClient::new(self.client)
    }

    pub fn forward<T: serde::Serialize>(
        &'a self,
        mail_folder_id: &str,
        comment: &str,
        to_recipients: &[T],
    ) -> ResponseClient<'a, I, GraphResponse<()>> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(format_mail_folder_message(mail_folder_id))
            .insert(UrlOrdering::Last("forward".into()))
            .set_body(
                serde_json::to_string_pretty(
                    &json!({ "comment": comment, "toRecipients": to_recipients }),
                )
                .unwrap(),
            );
        ResponseClient::new(self.client)
    }
}
