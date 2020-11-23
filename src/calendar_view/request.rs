use crate::calendar::CalendarRequest;
use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::instances::{InstanceRequest, InstancesRequest};
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::types::DeltaPhantom;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use graph_http::UploadSessionClient;
use handlebars::*;
use reqwest::Method;

register_client!(AttachmentsRequest,);
register_client!(CalendarViewRequest, ());
register_client!(CalendarViewsRequest,);

impl<'a, Client> AttachmentsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "# Invoke action createUploadSession",
        name: create_upload_session,
        response: UploadSessionClient<Client>,
        path: "/calendarView/{{RID}}/attachments/createUploadSession",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> CalendarViewRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn attachments(&self) -> AttachmentsRequest<'a, Client> {
        AttachmentsRequest::new(self.client)
    }
    pub fn calendar(&self) -> CalendarRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Calendar);
        CalendarRequest::new(self.client)
    }
    pub fn instances(&self) -> InstanceRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        InstanceRequest::new(self.client)
    }
    pub fn instance<ID: AsRef<str>>(&self, id: ID) -> InstancesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::Instances);
        InstancesRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get calendarView from me",
        name: get_calendar_view,
        response: serde_json::Value,
        path: "/calendarView/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property calendarView in me",
        name: update_calendar_view,
        response: GraphResponse<Content>,
        path: "/calendarView/{{RID}}",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> CalendarViewsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> CalendarViewRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::CalendarView);
        CalendarViewRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Get calendarView from me",
        name: list_calendar_view,
        response: Collection<serde_json::Value>,
        path: "/calendarView",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to calendarView for me",
        name: create_calendar_view,
        response: serde_json::Value,
        path: "/calendarView",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/calendarView/delta()",
        params: 0,
        has_body: false
    });
}
