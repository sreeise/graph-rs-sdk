use crate::client::Graph;
use crate::http::{
    AsyncClient, AsyncDownload, BlockingClient, BlockingDownload, GraphRequestType, GraphResponse,
    IntoResponse, RequestClient,
};
use crate::types::collection::Collection;
use crate::types::content::Content;
use graph_error::{AsRes, GraphRsError};
use handlebars::*;
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use reqwest::Method;
use std::ffi::OsStr;
use std::path::Path;

register_client!(
    OnenoteRequest,
    notebook => "onenote/notebooks",
    section => "onenote/sections",
    section_group => "onenote/sectionGroups",
    pages => "onenote/pages",
);

impl<'a, Client> OnenoteRequest<'a, Client>
where
    Client: crate::http::RequestClient,
{
    get!( list_sections, Collection<serde_json::Value> => "{{section}}" );
    get!( list_section_groups, Collection<serde_json::Value> => "{{section_group}}" );
    get!( list_pages, Collection<serde_json::Value> => "{{pages}}" );

    pub fn notebooks(&self) -> OnenoteNotebookRequest<'a, Client> {
        OnenoteNotebookRequest::new(self.client)
    }

    pub fn sections(&self) -> OnenoteSectionRequest<'a, Client> {
        OnenoteSectionRequest::new(self.client)
    }

    pub fn section_group(&self) -> OnenoteSectionGroupRequest<'a, Client> {
        OnenoteSectionGroupRequest::new(self.client)
    }

    pub fn pages(&self) -> OnenotePageRequest<'a, Client> {
        OnenotePageRequest::new(self.client)
    }

    pub fn create_page<P: AsRef<Path>>(
        &self,
        file: P,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
        render_path!(self.client, "{{pages}}");

        if !file.as_ref().extension().eq(&Some(OsStr::new("html"))) {
            return IntoResponse::new_error(
                self.client,
                GraphRsError::InvalidFileExtension {
                    requires: "html".to_string(),
                    found: file
                        .as_ref()
                        .extension()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                }
                .as_failure(),
            );
        }

        if let Err(e) = self.client.client().set_body_with_file(file) {
            return IntoResponse::new_error(self.client, e);
        }
        self.client
            .client()
            .header(CONTENT_TYPE, HeaderValue::from_static("text/html"))
            .set_method(Method::POST);

        IntoResponse::new(self.client)
    }
}

register_client!(OnenoteNotebookRequest,);

impl<'a, Client> OnenoteNotebookRequest<'a, Client>
where
    Client: crate::http::RequestClient,
{
    get!( list, Collection<serde_json::Value> => "{{notebook}}" );
    get!( | list_sections, Collection<serde_json::Value> => "{{notebook}}/{{id}}/sections" );
    get!( | get, serde_json::Value => "{{notebook}}/{{id}}" );
    post!( [ create, serde_json::Value => "{{notebook}}" ]);
    post!( [ | copy, serde_json::Value => "{{notebook}}/{{id}}/copyNotebook" ] );
    post!( [ | create_section, serde_json::Value => "{{notebook}}/{{id}}/sections" ] );

    pub fn recent(
        &self,
        include_personal_notebooks: bool,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
        render_path!(
            self.client,
            format!(
                "{{{{notebook}}}}/getRecentNotebooks(includePersonalNotebooks={})",
                include_personal_notebooks
            ).as_str()
        );
        self.client.client().set_method(Method::GET);
        IntoResponse::new(self.client)
    }
}

register_client!(OnenoteSectionRequest,);

impl<'a, Client> OnenoteSectionRequest<'a, Client>
where
    Client: crate::http::RequestClient,
{
    get!( list, Collection<serde_json::Value> => "{{section}}" );
    get!( | list_pages, Collection<serde_json::Value> => "{{section}}/{{id}}/pages" );
    get!( | get, serde_json::Value => "{{section}}/{{id}}" );
    post!( [ | copy_to_notebook, GraphResponse<Content> => "{{section}}/{{id}}/copyToNotebook" ] );
    post!( [ | copy_to_section_group, GraphResponse<Content> => "{{section}}/{{id}}/copyToSectionGroup" ] );

    pub fn create_page<S: AsRef<str>, P: AsRef<Path>>(
        &self,
        id: S,
        file: P,
    ) -> IntoResponse<'a, serde_json::Value, Client> {
        render_path!(
            self.client,
            "{{section}}/{{id}}/pages",
            &serde_json::json!({ "id": id.as_ref() })
        );

        if !file.as_ref().extension().eq(&Some(OsStr::new("html"))) {
            return IntoResponse::new_error(
                self.client,
                GraphRsError::InvalidFileExtension {
                    requires: "html".to_string(),
                    found: file
                        .as_ref()
                        .extension()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                }
                .as_failure(),
            );
        }

        if let Err(e) = self.client.client().set_body_with_file(file) {
            return IntoResponse::new_error(self.client, e);
        }
        self.client
            .client()
            .header(CONTENT_TYPE, HeaderValue::from_static("text/html"))
            .set_method(Method::POST);
        IntoResponse::new(self.client)
    }
}

register_client!(OnenoteSectionGroupRequest,);

impl<'a, Client> OnenoteSectionGroupRequest<'a, Client>
where
    Client: crate::http::RequestClient,
{
    get!( list, Collection<serde_json::Value> => "{{section_group}}" );
    get!( | list_sections, Collection<serde_json::Value> => "{{section_group}}/{{id}}/sections" );
    get!( | get, serde_json::Value => "{{section_group}}/{{id}}" );
    post!( [ | create, serde_json::Value => "{{section_group}}/{{id}}/sectionGroups" ] );
    post!( [ | create_section, serde_json::Value => "{{section_group}}/{{id}}/sections" ] );
}

register_client!(OnenotePageRequest,);

impl<'a, Client> OnenotePageRequest<'a, Client>
where
    Client: crate::http::RequestClient,
{
    get!( list, Collection<serde_json::Value> => "{{pages}}" );
    get!( | get, serde_json::Value => "{{pages}}/{{id}}" );
    get!( | content, GraphResponse<Content> => "{{pages}}/{{id}}/content" );
    patch!( [ | update, serde_json::Value => "{{pages}}/{{id}}/content" ] );
    post!( [ | copy_to_section, GraphResponse<Content> => "{{pages}}/{{id}}/copyToSection" ] );
    delete!( | delete, GraphResponse<Content> => "{{pages}}/{{id}}" );
}

impl<'a> OnenotePageRequest<'a, BlockingClient> {
    download!( | download, BlockingDownload => "{{pages}}/{{id}}/content" );
}

impl<'a> OnenotePageRequest<'a, AsyncClient> {
    download!( | download, AsyncDownload => "{{pages}}/{{id}}/content" );
}

/*

 pub fn download<S: AsRef<str>, P: AsRef<Path>>(
     &'a self,
     id: S,
     directory: P,
 ) ->BlockingDownload {
     render_path!(
         self.client,
         "{{pages}}/{{id}}/content",
         &serde_json::json!({ "id": id.as_ref() })
     );
     self.client
         .client()
         .set_method(Method::GET)
         .set_download_dir(directory.as_ref())
         .set_request_type(GraphRequestType::Redirect);
     self.client.request().download(self.client.take_builder())
 }
*/
