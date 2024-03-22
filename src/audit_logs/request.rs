// GENERATED CODE

use crate::api_default_imports::*;

api_client!(AuditLogsApiClient, ResourceIdentity::AuditLogs);

impl AuditLogsApiClient {
    get!(
        doc: "Get auditLogs",
        name: get_audit_log_root,
        path: "/auditLogs"
    );
    patch!(
        doc: "Update auditLogs",
        name: update_audit_log_root,
        path: "/auditLogs",
        body: true
    );
    post!(
        doc: "Create new navigation property to directoryAudits for auditLogs",
        name: create_directory_audits,
        path: "/auditLogs/directoryAudits",
        body: true
    );
    get!(
        doc: "List directoryAudits",
        name: list_directory_audits,
        path: "/auditLogs/directoryAudits"
    );
    get!(
        doc: "Get the number of the resource",
        name: directory_audits_fedb,
        path: "/auditLogs/directoryAudits/$count"
    );
    delete!(
        doc: "Delete navigation property directoryAudits for auditLogs",
        name: delete_directory_audits,
        path: "/auditLogs/directoryAudits/{{id}}",
        params: directory_audit_id
    );
    get!(
        doc: "Get directoryAudits from auditLogs",
        name: get_directory_audits,
        path: "/auditLogs/directoryAudits/{{id}}",
        params: directory_audit_id
    );
    patch!(
        doc: "Update the navigation property directoryAudits in auditLogs",
        name: update_directory_audits,
        path: "/auditLogs/directoryAudits/{{id}}",
        body: true,
        params: directory_audit_id
    );
    post!(
        doc: "Create new navigation property to provisioning for auditLogs",
        name: create_provisioning,
        path: "/auditLogs/provisioning",
        body: true
    );
    get!(
        doc: "List provisioningObjectSummary",
        name: list_provisioning,
        path: "/auditLogs/provisioning"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_provisioning_count,
        path: "/auditLogs/provisioning/$count"
    );
    delete!(
        doc: "Delete navigation property provisioning for auditLogs",
        name: delete_provisioning,
        path: "/auditLogs/provisioning/{{id}}",
        params: provisioning_object_summary_id
    );
    get!(
        doc: "Get provisioning from auditLogs",
        name: get_provisioning,
        path: "/auditLogs/provisioning/{{id}}",
        params: provisioning_object_summary_id
    );
    patch!(
        doc: "Update the navigation property provisioning in auditLogs",
        name: update_provisioning,
        path: "/auditLogs/provisioning/{{id}}",
        body: true,
        params: provisioning_object_summary_id
    );
    post!(
        doc: "Create new navigation property to signIns for auditLogs",
        name: create_sign_ins,
        path: "/auditLogs/signIns",
        body: true
    );
    get!(
        doc: "List signIns",
        name: list_sign_ins,
        path: "/auditLogs/signIns"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_sign_ins_count,
        path: "/auditLogs/signIns/$count"
    );
    delete!(
        doc: "Delete navigation property signIns for auditLogs",
        name: delete_sign_ins,
        path: "/auditLogs/signIns/{{id}}",
        params: sign_in_id
    );
    get!(
        doc: "Get signIns from auditLogs",
        name: get_sign_ins,
        path: "/auditLogs/signIns/{{id}}",
        params: sign_in_id
    );
    patch!(
        doc: "Update the navigation property signIns in auditLogs",
        name: update_sign_ins,
        path: "/auditLogs/signIns/{{id}}",
        body: true,
        params: sign_in_id
    );
}
