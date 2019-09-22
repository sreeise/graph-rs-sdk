use crate::client::{Graph, Ident};
use crate::http::GraphResponse;
use crate::http::ResponseClient;
use crate::types::collection::Collection;
use crate::url::UrlOrdering;
use graph_rs_types::entitytypes::Message;
use reqwest::Method;
use serde_json::json;
use std::marker::PhantomData;

macro_rules! message_method {
    ( $name:ident, $I:ty, $method:expr, $ord:expr ) => {
      pub fn $name(&self) -> ResponseClient<'a, I, $I> {
        self.client
            .request()
            .set_method($method)
            .insert(UrlOrdering::ItemPath("messages".into()))
            .extend($ord);
        ResponseClient::new(self.client)
      }
    };
}

macro_rules! mail_folder_id_method {
    ( $name:ident, $I:ty, $method:expr, $ord:expr ) => {
      pub fn $name(&self, mail_folder_id: &str) -> ResponseClient<'a, I, $I> {
        self.client
            .request()
            .set_method($method)
            .extend($ord)
            .insert(UrlOrdering::ItemPath(format!("mailFolders/{}/messages", mail_folder_id)));
        ResponseClient::new(self.client)
      }
    };
}

pub struct MailRequest<'a, I> {
    client: &'a Graph,
    ident: PhantomData<I>,
}

impl<'a, I> MailRequest<'a, I> {
    pub fn new(client: &'a Graph) -> MailRequest<'a, I> {
        MailRequest {
            client,
            ident: PhantomData,
        }
    }

    pub fn messages(&'a self) -> MessageRequest<'a, I> {
        MessageRequest::new(self.client)
    }

    pub fn mail_folder(&'a self) -> MailFolderRequest<'a, I> {
        MailFolderRequest::new(self.client)
    }
}

pub struct MessageRequest<'a, I> {
    client: &'a Graph,
    ident: PhantomData<I>,
}

impl<'a, I> MessageRequest<'a, I> {
    pub fn new(client: &'a Graph) -> MessageRequest<'a, I> {
        MessageRequest {
            client,
            ident: PhantomData,
        }
    }

    message_method!(get, Message, Method::GET, vec![]);
    message_method!(delete, GraphResponse<()>, Method::DELETE, vec![]);
    message_method!(
        create_reply,
        Message,
        Method::POST,
        vec![UrlOrdering::Last("createReply".into())]
    );
    message_method!(
        create_reply_all,
        Message,
        Method::POST,
        vec![UrlOrdering::Last("createReplyAll".into())]
    );
    message_method!(
        create_forward,
        Message,
        Method::POST,
        vec![UrlOrdering::Last("createForward".into())]
    );
    message_method!(
        send_message,
        GraphResponse<()>,
        Method::POST,
        vec![UrlOrdering::Last("send".into())]
    );

    pub fn list(&'a self) -> ResponseClient<'a, I, Collection<Message>> {
        self.client
            .request()
            .set_method(Method::GET)
            .insert(UrlOrdering::Last("messages".into()));
        if self.client.ident().eq(&Ident::Me) {
            self.client.format_ord();
        }
        ResponseClient::new(self.client)
    }

    pub fn update<B: serde::Serialize>(&'a self, body: &B) -> ResponseClient<'a, I, Message> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(UrlOrdering::ItemPath("messages".into()))
            .set_body(serde_json::to_string_pretty(body).unwrap());
        ResponseClient::new(self.client)
    }

    pub fn create<B: serde::Serialize>(&'a self, body: &B) -> ResponseClient<'a, I, Message> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(UrlOrdering::ItemPath("messages".into()))
            .set_body(serde_json::to_string_pretty(body).unwrap());
        ResponseClient::new(self.client)
    }

    pub fn copy(&'a self, destination_id: &str) -> ResponseClient<'a, I, Message> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(UrlOrdering::ItemPath("messages".into()))
            .insert(UrlOrdering::Last("copy".into()))
            .set_body(
                serde_json::to_string_pretty(&json!({ "destination_id": destination_id })).unwrap(),
            );
        ResponseClient::new(self.client)
    }

    pub fn move_message(&'a self, destination_id: &str) -> ResponseClient<'a, I, Message> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(UrlOrdering::ItemPath("messages".into()))
            .insert(UrlOrdering::Last("move".into()))
            .set_body(
                serde_json::to_string_pretty(&json!({ "destination_id": destination_id })).unwrap(),
            );
        ResponseClient::new(self.client)
    }

    pub fn reply(&'a self, comment: &str) -> ResponseClient<'a, I, GraphResponse<()>> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(UrlOrdering::ItemPath("messages".into()))
            .insert(UrlOrdering::Last("reply".into()))
            .set_body(serde_json::to_string_pretty(&json!({ "comment": comment })).unwrap());
        ResponseClient::new(self.client)
    }

    pub fn reply_all(&'a self, comment: &str) -> ResponseClient<'a, I, GraphResponse<()>> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(UrlOrdering::ItemPath("messages".into()))
            .insert(UrlOrdering::Last("replyAll".into()))
            .set_body(serde_json::to_string_pretty(&json!({ "comment": comment })).unwrap());
        ResponseClient::new(self.client)
    }

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

    pub fn send_mail<B: serde::Serialize>(
        &'a self,
        body: &B,
    ) -> ResponseClient<'a, I, GraphResponse<()>> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(UrlOrdering::Last("sendMail".into()))
            .set_body(serde_json::to_string_pretty(body).unwrap());
        if self.client.ident().eq(&Ident::Me) {
            self.client.format_ord();
        }
        ResponseClient::new(self.client)
    }
}

pub struct MailFolderRequest<'a, I> {
    client: &'a Graph,
    ident: PhantomData<I>,
}

impl<'a, I> MailFolderRequest<'a, I> {
    pub fn new(client: &'a Graph) -> MailFolderRequest<'a, I> {
        MailFolderRequest {
            client,
            ident: PhantomData,
        }
    }

    mail_folder_id_method!(get, Message, Method::GET, vec![]);
    mail_folder_id_method!(delete, GraphResponse<()>, Method::DELETE, vec![]);
    mail_folder_id_method!(
        create_reply,
        Message,
        Method::POST,
        vec![UrlOrdering::Last("createReply".into())]
    );
    mail_folder_id_method!(
        create_reply_all,
        Message,
        Method::POST,
        vec![UrlOrdering::Last("createReplyAll".into())]
    );
    mail_folder_id_method!(
        create_forward,
        Message,
        Method::POST,
        vec![UrlOrdering::Last("createForward".into())]
    );
    mail_folder_id_method!(
        send_message,
        GraphResponse<()>,
        Method::POST,
        vec![UrlOrdering::Last("send".into())]
    );

    pub fn list(&self, mail_folder_id: &str) -> ResponseClient<'a, I, Collection<Message>> {
        self.client
            .request()
            .set_method(Method::GET)
            .insert(UrlOrdering::ItemPath(format!(
                "mailFolders/{}/messages",
                mail_folder_id
            )));
        if self.client.ident().eq(&Ident::Me) {
            self.client.format_ord();
        }
        ResponseClient::new(self.client)
    }

    pub fn update<B: serde::Serialize>(
        &self,
        mail_folder_id: &str,
        body: &B,
    ) -> ResponseClient<'a, I, Message> {
        self.client
            .request()
            .set_method(Method::PATCH)
            .insert(UrlOrdering::ItemPath(format!(
                "mailFolders/{}/messages",
                mail_folder_id
            )))
            .set_body(serde_json::to_string_pretty(body).unwrap());
        ResponseClient::new(self.client)
    }

    pub fn create<B: serde::Serialize>(
        &'a self,
        mail_folder_id: &str,
        body: &B,
    ) -> ResponseClient<'a, I, Message> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(UrlOrdering::ItemPath(format!(
                "mailFolders/{}/messages",
                mail_folder_id
            )))
            .set_body(serde_json::to_string_pretty(body).unwrap());
        if self.client.ident().eq(&Ident::Me) {
            self.client.format_ord();
        }
        ResponseClient::new(self.client)
    }

    pub fn copy(
        &'a self,
        mail_folder_id: &str,
        destination_id: &str,
    ) -> ResponseClient<'a, I, Message> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(UrlOrdering::ItemPath(format!(
                "mailFolders/{}/messages",
                mail_folder_id
            )))
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
            .insert(UrlOrdering::ItemPath(format!(
                "mailFolders/{}/messages",
                mail_folder_id
            )))
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
            .insert(UrlOrdering::ItemPath(format!(
                "mailFolders/{}/messages",
                mail_folder_id
            )))
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
            .insert(UrlOrdering::ItemPath(format!(
                "mailFolders/{}/messages",
                mail_folder_id
            )))
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
            .insert(UrlOrdering::ItemPath(format!(
                "mailFolders/{}/messages",
                mail_folder_id
            )))
            .insert(UrlOrdering::Last("forward".into()))
            .set_body(
                serde_json::to_string_pretty(
                    &json!({ "comment": comment, "toRecipients": to_recipients }),
                )
                .unwrap(),
            );
        ResponseClient::new(self.client)
    }

    pub fn send_mail<B: serde::Serialize>(
        &'a self,
        mail_folder_id: &str,
        body: &B,
    ) -> ResponseClient<'a, I, GraphResponse<()>> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(UrlOrdering::ItemPath(format!(
                "mailFolders/{}/sendMail",
                mail_folder_id
            )))
            .set_body(serde_json::to_string_pretty(body).unwrap());
        if self.client.ident().eq(&Ident::Me) {
            self.client.format_ord();
        }
        ResponseClient::new(self.client)
    }
}
