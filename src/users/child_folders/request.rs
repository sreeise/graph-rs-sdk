// GENERATED CODE

use crate::api_default_imports::*;
use crate::extended_properties::*;
use crate::users::*;

resource_api_client!(
    ChildFoldersApiClient,
    ChildFoldersIdApiClient,
    ResourceIdentity::ChildFolders
);

impl ChildFoldersApiClient {
    post!(
        doc: "Create child folder",
        name: create_child_folders,
        path: "/childFolders",
        body: true
    );
    get!(
        doc: "List childFolders",
        name: list_child_folders,
        path: "/childFolders"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_child_folders_count,
        path: "/childFolders/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/childFolders/delta()"
    );
}

impl ChildFoldersIdApiClient {
    api_client_link_id!(child_folder, ChildFoldersIdApiClient);
    api_client_link!(extended_properties, ExtendedPropertiesApiClient);
    api_client_link_id!(messages_id, UsersMessagesIdApiClient);
    api_client_link!(messages, UsersMessagesApiClient);

    delete!(
        doc: "Delete navigation property childFolders for users",
        name: delete_child_folders,
        path: "/childFolders/{{RID}}"
    );
    get!(
        doc: "Get childFolders from users",
        name: get_child_folders,
        path: "/childFolders/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property childFolders in users",
        name: update_child_folders,
        path: "/childFolders/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action copy",
        name: copy,
        path: "/childFolders/{{RID}}/copy",
        body: true
    );
    post!(
        doc: "Create rule",
        name: create_message_rules,
        path: "/childFolders/{{RID}}/messageRules",
        body: true
    );
    get!(
        doc: "List rules",
        name: list_message_rules,
        path: "/childFolders/{{RID}}/messageRules"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_message_rules_count,
        path: "/childFolders/{{RID}}/messageRules/$count"
    );
    delete!(
        doc: "Delete navigation property messageRules for users",
        name: delete_message_rules,
        path: "/childFolders/{{RID}}/messageRules/{{id}}",
        params: message_rule_id
    );
    get!(
        doc: "Get messageRules from users",
        name: get_message_rules,
        path: "/childFolders/{{RID}}/messageRules/{{id}}",
        params: message_rule_id
    );
    patch!(
        doc: "Update the navigation property messageRules in users",
        name: update_message_rules,
        path: "/childFolders/{{RID}}/messageRules/{{id}}",
        body: true,
        params: message_rule_id
    );
    post!(
        doc: "Invoke action move",
        name: move_child_folders,
        path: "/childFolders/{{RID}}/move",
        body: true
    );
}
