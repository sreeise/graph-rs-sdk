// GENERATED CODE

use crate::api_default_imports::*;

api_client!(AdminApiClient, ResourceIdentity::Admin);

impl AdminApiClient {
    get!(
        doc: "Get admin",
        name: get_admin,
        path: "/admin"
    );
    patch!(
        doc: "Update admin",
        name: update_admin,
        path: "/admin",
        body: true
    );
    delete!(
        doc: "Delete navigation property serviceAnnouncement for admin",
        name: delete_service_announcement,
        path: "/admin/serviceAnnouncement"
    );
    get!(
        doc: "Get serviceAnnouncement from admin",
        name: get_service_announcement,
        path: "/admin/serviceAnnouncement"
    );
    patch!(
        doc: "Update the navigation property serviceAnnouncement in admin",
        name: update_service_announcement,
        path: "/admin/serviceAnnouncement",
        body: true
    );
    post!(
        doc: "Create new navigation property to healthOverviews for admin",
        name: create_health_overviews,
        path: "/admin/serviceAnnouncement/healthOverviews",
        body: true
    );
    get!(
        doc: "List healthOverviews",
        name: list_health_overviews,
        path: "/admin/serviceAnnouncement/healthOverviews"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_health_overviews_count,
        path: "/admin/serviceAnnouncement/healthOverviews/$count"
    );
    delete!(
        doc: "Delete navigation property healthOverviews for admin",
        name: delete_health_overviews,
        path: "/admin/serviceAnnouncement/healthOverviews/{{id}}",
        params: service_health_id
    );
    get!(
        doc: "Get healthOverviews from admin",
        name: get_health_overviews,
        path: "/admin/serviceAnnouncement/healthOverviews/{{id}}",
        params: service_health_id
    );
    patch!(
        doc: "Update the navigation property healthOverviews in admin",
        name: update_health_overviews,
        path: "/admin/serviceAnnouncement/healthOverviews/{{id}}",
        body: true,
        params: service_health_id
    );
    post!(
        doc: "Create new navigation property to issues for admin",
        name: create_health_overview_issues,
        path: "/admin/serviceAnnouncement/healthOverviews/{{id}}/issues",
        body: true,
        params: service_health_id
    );
    get!(
        doc: "Get issues from admin",
        name: list_health_overview_issues,
        path: "/admin/serviceAnnouncement/healthOverviews/{{id}}/issues",
        params: service_health_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_health_overview_issues_count,
        path: "/admin/serviceAnnouncement/healthOverviews/{{id}}/issues/$count",
        params: service_health_id
    );
    delete!(
        doc: "Delete navigation property issues for admin",
        name: delete_health_overview_issues,
        path: "/admin/serviceAnnouncement/healthOverviews/{{id}}/issues/{{id2}}",
        params: service_health_id, service_health_issue_id
    );
    get!(
        doc: "Get issues from admin",
        name: get_health_overview_issues,
        path: "/admin/serviceAnnouncement/healthOverviews/{{id}}/issues/{{id2}}",
        params: service_health_id, service_health_issue_id
    );
    patch!(
        doc: "Update the navigation property issues in admin",
        name: update_health_overview_issues,
        path: "/admin/serviceAnnouncement/healthOverviews/{{id}}/issues/{{id2}}",
        body: true,
        params: service_health_id, service_health_issue_id
    );
    get!(
        doc: "Invoke function incidentReport",
        name: health_overviews_incident_report,
        path: "/admin/serviceAnnouncement/healthOverviews/{{id}}/issues/{{id2}}/microsoft.graph.incidentReport()",
        params: service_health_id, service_health_issue_id
    );
    post!(
        doc: "Create new navigation property to issues for admin",
        name: create_issues,
        path: "/admin/serviceAnnouncement/issues",
        body: true
    );
    get!(
        doc: "List issues",
        name: list_issues,
        path: "/admin/serviceAnnouncement/issues"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_issues_count,
        path: "/admin/serviceAnnouncement/issues/$count"
    );
    delete!(
        doc: "Delete navigation property issues for admin",
        name: delete_issues,
        path: "/admin/serviceAnnouncement/issues/{{id}}",
        params: service_health_issue_id
    );
    get!(
        doc: "Get issues from admin",
        name: get_issues,
        path: "/admin/serviceAnnouncement/issues/{{id}}",
        params: service_health_issue_id
    );
    patch!(
        doc: "Update the navigation property issues in admin",
        name: update_issues,
        path: "/admin/serviceAnnouncement/issues/{{id}}",
        body: true,
        params: service_health_issue_id
    );
    get!(
        doc: "Invoke function incidentReport",
        name: incident_report,
        path: "/admin/serviceAnnouncement/issues/{{id}}/microsoft.graph.incidentReport()",
        params: service_health_issue_id
    );
    post!(
        doc: "Create new navigation property to messages for admin",
        name: create_messages,
        path: "/admin/serviceAnnouncement/messages",
        body: true
    );
    get!(
        doc: "List serviceAnnouncement messages",
        name: list_messages,
        path: "/admin/serviceAnnouncement/messages"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_messages_count,
        path: "/admin/serviceAnnouncement/messages/$count"
    );
    post!(
        doc: "Invoke action archive",
        name: archive,
        path: "/admin/serviceAnnouncement/messages/microsoft.graph.archive",
        body: true
    );
    post!(
        doc: "Invoke action favorite",
        name: favorite,
        path: "/admin/serviceAnnouncement/messages/microsoft.graph.favorite",
        body: true
    );
    post!(
        doc: "Invoke action markRead",
        name: mark_read,
        path: "/admin/serviceAnnouncement/messages/microsoft.graph.markRead",
        body: true
    );
    post!(
        doc: "Invoke action markUnread",
        name: mark_unread,
        path: "/admin/serviceAnnouncement/messages/microsoft.graph.markUnread",
        body: true
    );
    post!(
        doc: "Invoke action unarchive",
        name: unarchive,
        path: "/admin/serviceAnnouncement/messages/microsoft.graph.unarchive",
        body: true
    );
    post!(
        doc: "Invoke action unfavorite",
        name: unfavorite,
        path: "/admin/serviceAnnouncement/messages/microsoft.graph.unfavorite",
        body: true
    );
    delete!(
        doc: "Delete navigation property messages for admin",
        name: delete_messages,
        path: "/admin/serviceAnnouncement/messages/{{id}}",
        params: service_update_message_id
    );
    get!(
        doc: "Get messages from admin",
        name: get_messages,
        path: "/admin/serviceAnnouncement/messages/{{id}}",
        params: service_update_message_id
    );
    patch!(
        doc: "Update the navigation property messages in admin",
        name: update_messages,
        path: "/admin/serviceAnnouncement/messages/{{id}}",
        body: true,
        params: service_update_message_id
    );
    post!(
        doc: "Create new navigation property to attachments for admin",
        name: create_attachments,
        path: "/admin/serviceAnnouncement/messages/{{id}}/attachments",
        body: true,
        params: service_update_message_id
    );
    get!(
        doc: "List attachments",
        name: list_attachments,
        path: "/admin/serviceAnnouncement/messages/{{id}}/attachments",
        params: service_update_message_id
    );
    get!(
        doc: "Get the number of the resource",
        name: get_attachments_count,
        path: "/admin/serviceAnnouncement/messages/{{id}}/attachments/$count",
        params: service_update_message_id
    );
    delete!(
        doc: "Delete navigation property attachments for admin",
        name: delete_attachments,
        path: "/admin/serviceAnnouncement/messages/{{id}}/attachments/{{id2}}",
        params: service_update_message_id, service_announcement_attachment_id
    );
    get!(
        doc: "Get attachments from admin",
        name: get_attachments,
        path: "/admin/serviceAnnouncement/messages/{{id}}/attachments/{{id2}}",
        params: service_update_message_id, service_announcement_attachment_id
    );
    patch!(
        doc: "Update the navigation property attachments in admin",
        name: update_attachments,
        path: "/admin/serviceAnnouncement/messages/{{id}}/attachments/{{id2}}",
        body: true,
        params: service_update_message_id, service_announcement_attachment_id
    );
    get!(
        doc: "Get content for the navigation property attachments from admin",
        name: get_attachments_content,
        path: "/admin/serviceAnnouncement/messages/{{id}}/attachments/{{id2}}/content",
        params: service_update_message_id, service_announcement_attachment_id
    );
    put!(
        doc: "Update content for the navigation property attachments in admin",
        name: update_attachments_content,
        path: "/admin/serviceAnnouncement/messages/{{id}}/attachments/{{id2}}/content",
        body: true,
        params: service_update_message_id, service_announcement_attachment_id
    );
    get!(
        doc: "Get attachmentsArchive for the navigation property messages from admin",
        name: get_messages_attachments_archive,
        path: "/admin/serviceAnnouncement/messages/{{id}}/attachmentsArchive",
        params: service_update_message_id
    );
    put!(
        doc: "Update attachmentsArchive for the navigation property messages in admin",
        name: update_messages_attachments_archive,
        path: "/admin/serviceAnnouncement/messages/{{id}}/attachmentsArchive",
        body: true,
        params: service_update_message_id
    );
}
