use crate::client::Graph;
use graph_http::types::{Collection, Content};
use graph_http::{GraphResponse, IntoResponse};
use reqwest::Method;

register_client!(AttachmentRequest,);

impl<'a, Client> AttachmentRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( | get, serde_json::Value => "attachments/{{id}}" );
    get!( | content, GraphResponse<Content> => "attachments/{{id}}/$value" );
    delete!( | delete, GraphResponse<Content> => "attachments/{{id}}" );

    pub fn calendars(&'a self) -> CalendarAttachmentRequest<'a, Client> {
        CalendarAttachmentRequest::new(self.client)
    }

    pub fn calendar_groups(&'a self) -> CalendarGroupAttachmentRequest<'a, Client> {
        CalendarGroupAttachmentRequest::new(self.client)
    }

    pub fn mail_folder(&'a self) -> MailFolderMessageAttachmentRequest<'a, Client> {
        MailFolderMessageAttachmentRequest::new(self.client)
    }

    pub fn messages(&'a self) -> MailMessageAttachmentRequest<'a, Client> {
        MailMessageAttachmentRequest::new(self.client)
    }

    pub fn thread_posts(&'a self) -> ThreadPostAttachmentRequest<'a, Client> {
        ThreadPostAttachmentRequest::new(self.client)
    }

    pub fn conversation_posts(&'a self) -> ThreadConvoPostAttachmentRequest<'a, Client> {
        ThreadConvoPostAttachmentRequest::new(self.client)
    }
}

register_client!(CalendarAttachmentRequest,);

impl<'a, Client> CalendarAttachmentRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( || get_default, serde_json::Value => "events/{{id}}/attachments/{{id}}" );
    get!( || default_content, GraphResponse<Content> => "events/{{id}}/attachments/{{id}}/$value" );
    delete!( || delete_default, GraphResponse<Content> => "events/{{id}}/attachments/{{id}}" );
    get!( ||| get, serde_json::Value => "calendar/{{id}}/events/{{id2}}/attachments/{{id3}}" );
    get!( ||| content, GraphResponse<Content> => "calendar/{{id}}/events/{{id2}}/attachments/{{id3}}/$value" );
    delete!( ||| delete, GraphResponse<Content> => "calendar/{{id}}/events/{{id2}}/attachments/{{id3}}" );
}

register_client!(CalendarGroupAttachmentRequest,);

impl<'a, Client> CalendarGroupAttachmentRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( ||| get_default, serde_json::Value => "calendargroup/calendars/{{id}}/events/{{id2}}/attachments/{{id3}}" );
    get!( ||| default_content, GraphResponse<Content> => "calendargroup/calendars/{{id}}/events/{{id2}}/attachments/{{id3}}/$value" );
    delete!( ||| delete_default, GraphResponse<Content> => "calendargroup/calendars/{{id}}/events/{{id2}}/attachments/{{id3}}" );
    get!( |||| get, serde_json::Value => "calendargroups/{{id}}/calendars/{{id2}}/events/{{id3}}/attachments/{{id4}}" );
    get!( |||| content, GraphResponse<Content> => "calendargroups/{{id}}/calendars/{{id2}}/events/{{id3}}/attachments/{{id4}}/$value" );
    delete!( |||| delete, GraphResponse<Content> => "calendargroups/{{id}}/calendars/{{id2}}/events/{{id3}}/attachments/{{id4}}" );
}

register_client!(MailMessageAttachmentRequest,);

impl<'a, Client> MailMessageAttachmentRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( || get, serde_json::Value => "messages/{{id}}/attachments/{{id2}}" );
    post!( [ | add, serde_json::Value => "messages/{{id}}/attachments" ] );
    get!( || content, GraphResponse<Content> => "messages/{{id}}/attachments/{{id2}}/$value" );
    delete!( || delete, GraphResponse<Content> => "messages/{{id}}/attachments/{{id2}}" );

    pub fn mail_folder(&'a self) -> MailFolderMessageAttachmentRequest<'a, Client> {
        MailFolderMessageAttachmentRequest::new(self.client)
    }
}

register_client!(MailFolderMessageAttachmentRequest,);

impl<'a, Client> MailFolderMessageAttachmentRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( ||| get, serde_json::Value => "mailFolders/{{id}}/messages/{{id2}}/attachments/{{id3}}" );
    get!( ||| content, GraphResponse<Content> => "mailFolders/{{id}}/messages/{{id2}}/attachments/{{id3}}/$value" );
    post!( [ || add, serde_json::Value => "mailFolders/{{id}}/messages/{{id2}}/attachments" ] );
    delete!( ||| delete, GraphResponse<Content> => "mailFolders/{{id}}/messages/{{id2}}/attachments/{{id3}}" );

    fn render_child_folder_path<S: AsRef<str>>(
        &'a self,
        mail_folder_id: S,
        child_folders: &[&str],
        message_id: S,
        attachment_id: S,
        content: bool,
    ) {
        let vec: Vec<String> = child_folders
            .iter()
            .map(|s| format!("childFolders/{}/", s))
            .collect();

        let path = {
            if content {
                format!(
                    "mailFolders/{{{{id}}}}/{}/messages/{{{{id2}}}}/attachments/{{{{id3}}}}/$value",
                    vec.join("")
                )
            } else {
                format!(
                    "mailFolders/{{{{id}}}}/{}/messages/{{{{id2}}}}/attachments/{{{{id3}}}}",
                    vec.join("")
                )
            }
        };

        render_path!(
            self.client,
            path.as_str(),
            &serde_json::json!({
             "id": mail_folder_id.as_ref(),
             "id2": message_id.as_ref(),
             "id3": attachment_id.as_ref(),
            })
        );
    }

    pub fn child_folder<S: AsRef<str>>(
        &'a self,
        mail_folder_id: S,
        child_folders: &[&str],
        message_id: S,
        attachment_id: S,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
        self.client.request().set_method(Method::GET);
        self.render_child_folder_path(
            mail_folder_id,
            child_folders,
            message_id,
            attachment_id,
            false,
        );
        IntoResponse::new(&self.client.request)
    }

    pub fn child_folder_content<S: AsRef<str>>(
        &'a self,
        mail_folder_id: S,
        child_folders: &[&str],
        message_id: S,
        attachment_id: S,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
        self.client.request().set_method(Method::GET);
        self.render_child_folder_path(
            mail_folder_id,
            child_folders,
            message_id,
            attachment_id,
            true,
        );
        IntoResponse::new(&self.client.request)
    }

    pub fn delete_child_folder<S: AsRef<str>>(
        &'a self,
        mail_folder_id: S,
        child_folders: &[&str],
        message_id: S,
        attachment_id: S,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
        self.client.request().set_method(Method::DELETE);
        self.render_child_folder_path(
            mail_folder_id,
            child_folders,
            message_id,
            attachment_id,
            false,
        );
        IntoResponse::new(&self.client.request)
    }
}

register_client!(ThreadPostAttachmentRequest,);

impl<'a, Client> ThreadPostAttachmentRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( || list, Collection<serde_json::Value> => "threads/{{id}}/posts/{{id2}}/attachments" );
    get!( ||| get, serde_json::Value => "threads/{{id}}/posts/{{id2}}/attachments/{{id3}}" );
    get!( ||| content, GraphResponse<Content> => "threads/{{id}}/posts/{{id2}}/attachments/{{id3}}/$value" );
    delete!( ||| delete, GraphResponse<Content> => "threads/{{id}}/posts/{{id2}}/attachments/{{id3}}" );
}

register_client!(ThreadConvoPostAttachmentRequest,);

impl<'a, Client> ThreadConvoPostAttachmentRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!( ||| list, Collection<serde_json::Value> => "conversations/{{id}}/threads/{{id2}}/posts/{{id3}}/attachments" );
    get!( |||| get, serde_json::Value => "conversations/{{id}}/threads/{{id2}}/posts/{{id3}}/attachments/{{id4}}" );
    get!( |||| content, GraphResponse<Content> => "conversations/{{id}}/threads/{{id2}}/posts/{{id3}}/attachments/{{id4}}/$value" );
    delete!( |||| delete, GraphResponse<Content> => "conversations/{{id}}/threads/{{id2}}/posts/{{id3}}/attachments/{{id4}}" );
}
