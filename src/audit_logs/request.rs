use crate::client::Graph;
use graph_http::types::{Collection, Content};
use graph_http::{GraphResponse, IntoResponse};
use reqwest::Method;

register_client!(AuditLogsRequest,);
register_client!(AuditLogRootRequest,);

impl<'a, Client> AuditLogsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn audit_log_root(&self) -> AuditLogRootRequest<'a, Client> {
        AuditLogRootRequest::new(&self.client)
    }

    get!({
        doc: "# Get directoryAudits from auditLogs",
        name: get_directory_audits,
        response: serde_json::Value,
        path: "/auditLogs/directoryAudits/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property directoryAudits in auditLogs",
        name: update_directory_audits,
        response: GraphResponse<Content>,
        path: "/auditLogs/directoryAudits/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property directoryAudits for auditLogs",
        name: delete_directory_audits,
        response: GraphResponse<Content>,
        path: "/auditLogs/directoryAudits/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get restrictedSignIns from auditLogs",
        name: list_restricted_sign_ins,
        response: Collection<serde_json::Value>,
        path: "/auditLogs/restrictedSignIns",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to restrictedSignIns for auditLogs",
        name: create_restricted_sign_ins,
        response: serde_json::Value,
        path: "/auditLogs/restrictedSignIns",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get restrictedSignIns from auditLogs",
        name: get_restricted_sign_ins,
        response: serde_json::Value,
        path: "/auditLogs/restrictedSignIns/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property restrictedSignIns in auditLogs",
        name: update_restricted_sign_ins,
        response: GraphResponse<Content>,
        path: "/auditLogs/restrictedSignIns/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property restrictedSignIns for auditLogs",
        name: delete_restricted_sign_ins,
        response: GraphResponse<Content>,
        path: "/auditLogs/restrictedSignIns/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get directoryAudits from auditLogs",
        name: list_directory_audits,
        response: Collection<serde_json::Value>,
        path: "/auditLogs/directoryAudits",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to directoryAudits for auditLogs",
        name: create_directory_audits,
        response: serde_json::Value,
        path: "/auditLogs/directoryAudits",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get signIns from auditLogs",
        name: list_sign_ins,
        response: Collection<serde_json::Value>,
        path: "/auditLogs/signIns",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to signIns for auditLogs",
        name: create_sign_ins,
        response: serde_json::Value,
        path: "/auditLogs/signIns",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get signIns from auditLogs",
        name: get_sign_ins,
        response: serde_json::Value,
        path: "/auditLogs/signIns/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property signIns in auditLogs",
        name: update_sign_ins,
        response: GraphResponse<Content>,
        path: "/auditLogs/signIns/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property signIns for auditLogs",
        name: delete_sign_ins,
        response: GraphResponse<Content>,
        path: "/auditLogs/signIns/{{id}}",
        params: 1,
        has_body: false
    });
}

impl<'a, Client> AuditLogRootRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get auditLogs",
        name: get_audit_log_root,
        response: serde_json::Value,
        path: "/auditLogs",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update auditLogs",
        name: update_audit_log_root,
        response: GraphResponse<Content>,
        path: "/auditLogs",
        params: 0,
        has_body: true
    });
}
