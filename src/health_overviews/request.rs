// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;
use graph_http::{AsyncDownload, AsyncHttpClient, BlockingDownload, BlockingHttpClient};

register_client!(HealthOverviewsIdRequest, ());

impl<'a, Client> HealthOverviewsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Create new navigation property to issues for admin",
        name: create_issues,
        response: serde_json::Value,
        path: "/healthOverviews/{{RID}}/issues",
        has_body: true
    });
    get!({
        doc: "Get issues from admin",
        name: list_issues,
        response: serde_json::Value,
        path: "/healthOverviews/{{RID}}/issues",
        has_body: false
    });
    get!({
        doc: "Get issues from admin",
        name: get_issues,
        response: serde_json::Value,
        path: "/healthOverviews/{{RID}}/issues/{{id}}",
        params: [ service_health_issue_id ],
        has_body: false
    });
    patch!({
        doc: "Update the navigation property issues in admin",
        name: update_issues,
        response: NoContent,
        path: "/healthOverviews/{{RID}}/issues/{{id}}",
        params: [ service_health_issue_id ],
        has_body: true
    });
    delete!({
        doc: "Delete navigation property issues for admin",
        name: delete_issues,
        response: NoContent,
        path: "/healthOverviews/{{RID}}/issues/{{id}}",
        params: [ service_health_issue_id ],
        has_body: false
    });
}

impl<'a> HealthOverviewsIdRequest<'a, BlockingHttpClient> {
    download!({
        doc: "Invoke function incidentReport",
        name: incident_report,
        response: BlockingDownload,
        path: "/admin/serviceAnnouncement/healthOverviews/{{RID}}/issues/{{id}}/microsoft.graph.incidentReport()",
        params: [ service_health_issue_id ],
        has_body: false
    });
}

impl<'a> HealthOverviewsIdRequest<'a, AsyncHttpClient> {
    async_download!({
        doc: "Invoke function incidentReport",
        name: incident_report,
        response: AsyncDownload,
        path: "/admin/serviceAnnouncement/healthOverviews/{{RID}}/issues/{{id}}/microsoft.graph.incidentReport()",
        params: [ service_health_issue_id ],
        has_body: false
    });
}
