use crate::client::Graph;
use crate::http::{GraphResponse, IntoResponse};
use crate::types::collection::Collection;
use crate::types::content::Content;
use graph_rs_types::entitytypes::Attachment;
use reqwest::Method;
use std::marker::PhantomData;

register_client!(AttachmentRequest,);

impl<'a, I> AttachmentRequest<'a, I> {
    get!( | get, Attachment => "attachments/{{id}}" );
    get!( | content, GraphResponse<Content> => "attachments/{{id}}/$value" );
    delete!( | delete, GraphResponse<Content> => "attachments/{{id}}" );

    pub fn calendars(&'a self) -> CalendarAttachmentRequest<'a, I> {
        CalendarAttachmentRequest::new(self.client)
    }

    pub fn calendar_groups(&'a self) -> CalendarGroupAttachmentRequest<'a, I> {
        CalendarGroupAttachmentRequest::new(self.client)
    }

    pub fn mail_folder(&'a self) -> MailFolderMessageAttachmentRequest<'a, I> {
        MailFolderMessageAttachmentRequest::new(self.client)
    }

    pub fn messages(&'a self) -> MailMessageAttachmentRequest<'a, I> {
        MailMessageAttachmentRequest::new(self.client)
    }

    pub fn thread_posts(&'a self) -> ThreadPostAttachmentRequest<'a, I> {
        ThreadPostAttachmentRequest::new(self.client)
    }

    pub fn conversation_posts(&'a self) -> ThreadConvoPostAttachmentRequest<'a, I> {
        ThreadConvoPostAttachmentRequest::new(self.client)
    }
}

register_client!(CalendarAttachmentRequest,);

impl<'a, I> CalendarAttachmentRequest<'a, I> {
    get!( || get_default, Attachment => "events/{{id}}/attachments/{{id}}" );
    get!( || default_content, GraphResponse<Content> => "events/{{id}}/attachments/{{id}}/$value" );
    delete!( || delete_default, GraphResponse<Content> => "events/{{id}}/attachments/{{id}}" );
    get!( ||| get, Attachment => "calendar/{{id}}/events/{{id2}}/attachments/{{id3}}" );
    get!( ||| content, GraphResponse<Content> => "calendar/{{id}}/events/{{id2}}/attachments/{{id3}}/$value" );
    delete!( ||| delete, GraphResponse<Content> => "calendar/{{id}}/events/{{id2}}/attachments/{{id3}}" );
}

register_client!(CalendarGroupAttachmentRequest,);

impl<'a, I> CalendarGroupAttachmentRequest<'a, I> {
    get!( ||| get_default, Attachment => "calendargroup/calendars/{{id}}/events/{{id2}}/attachments/{{id3}}" );
    get!( ||| default_content, GraphResponse<Content> => "calendargroup/calendars/{{id}}/events/{{id2}}/attachments/{{id3}}/$value" );
    delete!( ||| delete_default, GraphResponse<Content> => "calendargroup/calendars/{{id}}/events/{{id2}}/attachments/{{id3}}" );
    get!( |||| get, Attachment => "calendargroups/{{id}}/calendars/{{id2}}/events/{{id3}}/attachments/{{id4}}" );
    get!( |||| content, GraphResponse<Content> => "calendargroups/{{id}}/calendars/{{id2}}/events/{{id3}}/attachments/{{id4}}/$value" );
    delete!( |||| delete, GraphResponse<Content> => "calendargroups/{{id}}/calendars/{{id2}}/events/{{id3}}/attachments/{{id4}}" );
}

register_client!(MailMessageAttachmentRequest,);

impl<'a, I> MailMessageAttachmentRequest<'a, I> {
    get!( || get, Attachment => "messages/{{id}}/attachments/{{id2}}" );
    get!( || content, GraphResponse<Content> => "messages/{{id}}/attachments/{{id2}}/$value" );
    delete!( || delete, GraphResponse<Content> => "messages/{{id}}/attachments/{{id2}}" );

    pub fn mail_folder(&'a self) -> MailFolderMessageAttachmentRequest<'a, I> {
        MailFolderMessageAttachmentRequest::new(self.client)
    }
}

register_client!(MailFolderMessageAttachmentRequest,);

impl<'a, I> MailFolderMessageAttachmentRequest<'a, I> {
    get!( ||| get, Attachment => "mailFolders/{{id}}/messages/{{id2}}/attachments/{{id3}}" );
    get!( ||| content, GraphResponse<Content> => "mailFolders/{{id}}/messages/{{id2}}/attachments/{{id3}}/$value" );
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
    ) -> IntoResponse<'a, I, Attachment> {
        self.client.builder().set_method(Method::GET);
        self.render_child_folder_path(
            mail_folder_id,
            child_folders,
            message_id,
            attachment_id,
            false,
        );
        IntoResponse::new(self.client)
    }

    pub fn child_folder_content<S: AsRef<str>>(
        &'a self,
        mail_folder_id: S,
        child_folders: &[&str],
        message_id: S,
        attachment_id: S,
    ) -> IntoResponse<'a, I, Attachment> {
        self.client.builder().set_method(Method::GET);
        self.render_child_folder_path(
            mail_folder_id,
            child_folders,
            message_id,
            attachment_id,
            true,
        );
        IntoResponse::new(self.client)
    }

    pub fn delete_child_folder<S: AsRef<str>>(
        &'a self,
        mail_folder_id: S,
        child_folders: &[&str],
        message_id: S,
        attachment_id: S,
    ) -> IntoResponse<'a, I, Attachment> {
        self.client.builder().set_method(Method::DELETE);
        self.render_child_folder_path(
            mail_folder_id,
            child_folders,
            message_id,
            attachment_id,
            false,
        );
        IntoResponse::new(self.client)
    }
}

register_client!(ThreadPostAttachmentRequest,);

impl<'a, I> ThreadPostAttachmentRequest<'a, I> {
    get!( || list, Collection<Attachment> => "threads/{{id}}/posts/{{id2}}/attachments" );
    get!( ||| get, Attachment => "threads/{{id}}/posts/{{id2}}/attachments/{{id3}}" );
    get!( ||| content, GraphResponse<Content> => "threads/{{id}}/posts/{{id2}}/attachments/{{id3}}/$value" );
    delete!( ||| delete, GraphResponse<Content> => "threads/{{id}}/posts/{{id2}}/attachments/{{id3}}" );
}

register_client!(ThreadConvoPostAttachmentRequest,);

impl<'a, I> ThreadConvoPostAttachmentRequest<'a, I> {
    get!( ||| list, Collection<Attachment> => "conversations/{{id}}/threads/{{id2}}/posts/{{id3}}/attachments" );
    get!( |||| get, Attachment => "conversations/{{id}}/threads/{{id2}}/posts/{{id3}}/attachments/{{id4}}" );
    get!( |||| content, GraphResponse<Content> => "conversations/{{id}}/threads/{{id2}}/posts/{{id3}}/attachments/{{id4}}/$value" );
    delete!( |||| delete, GraphResponse<Content> => "conversations/{{id}}/threads/{{id2}}/posts/{{id3}}/attachments/{{id4}}" );
}
