// GENERATED CODE

use crate::api_default_imports::*;
use crate::health_overviews::HealthOverviewsIdRequest;
use crate::messages::{MessageRequest, MessagesRequest};
use graph_http::types::NoContent;
use graph_http::{AsyncDownload, AsyncHttpClient, BlockingDownload, BlockingHttpClient};

register_client!(ServiceAnnouncementRequest,);

impl<'a, Client> ServiceAnnouncementRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn health_overview<ID: AsRef<str>>(&self, id: ID) -> HealthOverviewsIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::HealthOverviews);
        HealthOverviewsIdRequest::new(id.as_ref(), self.client)
    }

    pub fn messages(&self) -> MessageRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        MessageRequest::new(self.client)
    }

    pub fn message<ID: AsRef<str>>(&self, id: ID) -> MessagesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Messages);
        MessagesRequest::new(id.as_ref(), self.client)
    }

    post!({
        doc: "Create new navigation property to issues for admin",
        name: create_issues,
        response: serde_json::Value,
        path: "/serviceAnnouncement/issues",
        has_body: true
    });
    get!({
        doc: "Get issues from admin",
        name: list_issues,
        response: serde_json::Value,
        path: "/serviceAnnouncement/issues",
        has_body: false
    });
    get!({
        doc: "Get issues from admin",
        name: get_issues,
        response: serde_json::Value,
        path: "/serviceAnnouncement/issues/{{id}}",
        params: [ service_health_issue_id ],
        has_body: false
    });
    delete!({
        doc: "Delete navigation property issues for admin",
        name: delete_issues,
        response: NoContent,
        path: "/serviceAnnouncement/issues/{{id}}",
        params: [ service_health_issue_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property issues in admin",
        name: update_issues,
        response: NoContent,
        path: "/serviceAnnouncement/issues/{{id}}",
        params: [ service_health_issue_id ],
        has_body: true
    });

    get!({
        doc: "Get healthOverviews from admin",
        name: list_health_overviews,
        response: serde_json::Value,
        path: "/serviceAnnouncement/healthOverviews",
        has_body: false
    });
    post!({
        doc: "Create new navigation property to healthOverviews for admin",
        name: create_health_overviews,
        response: serde_json::Value,
        path: "/serviceAnnouncement/healthOverviews",
        has_body: true
    });
}

impl<'a> ServiceAnnouncementRequest<'a, BlockingHttpClient> {
    download!({
        doc: "Invoke function incidentReport",
        name: incident_report,
        response: BlockingDownload,
        path: "/serviceAnnouncement/issues/{{id}}/microsoft.graph.incidentReport()",
        params: [ service_health_issue_id ],
        has_body: false
    });
}

impl<'a> ServiceAnnouncementRequest<'a, AsyncHttpClient> {
    async_download!({
        doc: "Invoke function incidentReport",
        name: incident_report,
        response: AsyncDownload,
        path: "/serviceAnnouncement/issues/{{id}}/microsoft.graph.incidentReport()",
        params: [ service_health_issue_id ],
        has_body: false
    });
}
