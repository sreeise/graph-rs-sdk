// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    TodoListsTasksApiClient,
    TodoListsTasksIdApiClient,
    ResourceIdentity::TodoListsTasks
);

impl TodoListsTasksApiClient {
    post!(
        doc: "Create todoTask",
        name: create_tasks,
        path: "/tasks",
        body: true
    );
    get!(
        doc: "List tasks",
        name: list_tasks,
        path: "/tasks"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_tasks_count,
        path: "/tasks/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/tasks/delta()"
    );
}

impl TodoListsTasksIdApiClient {
    delete!(
        doc: "Delete navigation property tasks for users",
        name: delete_tasks,
        path: "/tasks/{{RID}}"
    );
    get!(
        doc: "Get tasks from users",
        name: get_tasks,
        path: "/tasks/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property tasks in users",
        name: update_tasks,
        path: "/tasks/{{RID}}",
        body: true
    );
    get!(
        doc: "Get attachmentSessions from users",
        name: list_attachment_sessions,
        path: "/tasks/{{RID}}/attachmentSessions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_attachment_sessions_count,
        path: "/tasks/{{RID}}/attachmentSessions/$count"
    );
    delete!(
        doc: "Delete navigation property attachmentSessions for users",
        name: delete_attachment_sessions,
        path: "/tasks/{{RID}}/attachmentSessions/{{id}}",
        params: attachment_session_id
    );
    get!(
        doc: "Get attachmentSessions from users",
        name: get_attachment_sessions,
        path: "/tasks/{{RID}}/attachmentSessions/{{id}}",
        params: attachment_session_id
    );
    patch!(
        doc: "Update the navigation property attachmentSessions in users",
        name: update_attachment_sessions,
        path: "/tasks/{{RID}}/attachmentSessions/{{id}}",
        body: true,
        params: attachment_session_id
    );
    get!(
        doc: "Get content for the navigation property attachmentSessions from users",
        name: get_attachment_sessions_content,
        path: "/tasks/{{RID}}/attachmentSessions/{{id}}/content",
        params: attachment_session_id
    );
    put!(
        doc: "Update content for the navigation property attachmentSessions in users",
        name: update_attachment_sessions_content,
        path: "/tasks/{{RID}}/attachmentSessions/{{id}}/content",
        body: true,
        params: attachment_session_id
    );
    post!(
        doc: "Create taskFileAttachment",
        name: create_attachments,
        path: "/tasks/{{RID}}/attachments",
        body: true
    );
    get!(
        doc: "List taskFileAttachments",
        name: list_attachments,
        path: "/tasks/{{RID}}/attachments"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_attachments_count,
        path: "/tasks/{{RID}}/attachments/$count"
    );
    post!(
        doc: "Invoke action createUploadSession",
        name: create_upload_session,
        path: "/tasks/{{RID}}/attachments/createUploadSession",
        body: true
    );
    delete!(
        doc: "Delete navigation property attachments for users",
        name: delete_attachments,
        path: "/tasks/{{RID}}/attachments/{{id}}",
        params: attachment_base_id
    );
    get!(
        doc: "Get attachments from users",
        name: get_attachments,
        path: "/tasks/{{RID}}/attachments/{{id}}",
        params: attachment_base_id
    );
    get!(
        doc: "Get media content for the navigation property attachments from users",
        name: get_attachments_content,
        path: "/tasks/{{RID}}/attachments/{{id}}/$value",
        params: attachment_base_id
    );
    put!(
        doc: "Update media content for the navigation property attachments in users",
        name: update_attachments_content,
        path: "/tasks/{{RID}}/attachments/{{id}}/$value",
        body: true,
        params: attachment_base_id
    );
    post!(
        doc: "Create new navigation property to checklistItems for users",
        name: create_checklist_items,
        path: "/tasks/{{RID}}/checklistItems",
        body: true
    );
    get!(
        doc: "Get checklistItems from users",
        name: list_checklist_items,
        path: "/tasks/{{RID}}/checklistItems"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_checklist_items_count,
        path: "/tasks/{{RID}}/checklistItems/$count"
    );
    delete!(
        doc: "Delete navigation property checklistItems for users",
        name: delete_checklist_items,
        path: "/tasks/{{RID}}/checklistItems/{{id}}",
        params: checklist_item_id
    );
    get!(
        doc: "Get checklistItems from users",
        name: get_checklist_items,
        path: "/tasks/{{RID}}/checklistItems/{{id}}",
        params: checklist_item_id
    );
    patch!(
        doc: "Update the navigation property checklistItems in users",
        name: update_checklist_items,
        path: "/tasks/{{RID}}/checklistItems/{{id}}",
        body: true,
        params: checklist_item_id
    );
    post!(
        doc: "Create new navigation property to extensions for users",
        name: create_extensions,
        path: "/tasks/{{RID}}/extensions",
        body: true
    );
    get!(
        doc: "Get extensions from users",
        name: list_extensions,
        path: "/tasks/{{RID}}/extensions"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_extensions_count,
        path: "/tasks/{{RID}}/extensions/$count"
    );
    delete!(
        doc: "Delete navigation property extensions for users",
        name: delete_extensions,
        path: "/tasks/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    get!(
        doc: "Get extensions from users",
        name: get_extensions,
        path: "/tasks/{{RID}}/extensions/{{id}}",
        params: extension_id
    );
    patch!(
        doc: "Update the navigation property extensions in users",
        name: update_extensions,
        path: "/tasks/{{RID}}/extensions/{{id}}",
        body: true,
        params: extension_id
    );
    post!(
        doc: "Create linkedResource",
        name: create_linked_resources,
        path: "/tasks/{{RID}}/linkedResources",
        body: true
    );
    get!(
        doc: "List linkedResources",
        name: list_linked_resources,
        path: "/tasks/{{RID}}/linkedResources"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_linked_resources_count,
        path: "/tasks/{{RID}}/linkedResources/$count"
    );
    delete!(
        doc: "Delete navigation property linkedResources for users",
        name: delete_linked_resources,
        path: "/tasks/{{RID}}/linkedResources/{{id}}",
        params: linked_resource_id
    );
    get!(
        doc: "Get linkedResources from users",
        name: get_linked_resources,
        path: "/tasks/{{RID}}/linkedResources/{{id}}",
        params: linked_resource_id
    );
    patch!(
        doc: "Update the navigation property linkedResources in users",
        name: update_linked_resources,
        path: "/tasks/{{RID}}/linkedResources/{{id}}",
        body: true,
        params: linked_resource_id
    );
}
