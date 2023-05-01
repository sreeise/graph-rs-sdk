// GENERATED CODE

use crate::api_default_imports::*;
use crate::extended_properties::*;
use crate::users::*;

api_client!(
    MailFoldersApiClient,
    MailFoldersIdApiClient,
    ResourceIdentity::MailFolders
);

impl MailFoldersApiClient {
    post!(
        doc: "Create MailFolder",
        name: create_mail_folders,
        path: "/mailFolders",
        body: true
    );
    get!(
        doc: "List mailFolders",
        name: list_mail_folders,
        path: "/mailFolders"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_mail_folders_count,
        path: "/mailFolders/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/mailFolders/delta()"
    );
}

impl MailFoldersIdApiClient {
    api_client_link!(child_folders, ChildFoldersApiClient);
    api_client_link_id!(child_folder, ChildFoldersIdApiClient);
    api_client_link!(messages, UsersMessagesApiClient);
    api_client_link_id!(messages_id, UsersMessagesIdApiClient);
    api_client_link!(extended_properties, ExtendedPropertiesApiClient);

    delete!(
        doc: "Delete navigation property mailFolders for users",
        name: delete_mail_folders,
        path: "/mailFolders/{{RID}}"
    );
    get!(
        doc: "Get mailFolders from users",
        name: get_mail_folders,
        path: "/mailFolders/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property mailFolders in users",
        name: update_mail_folders,
        path: "/mailFolders/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action copy",
        name: copy,
        path: "/mailFolders/{{RID}}/copy",
        body: true
    );
    post!(
        doc: "Create rule",
        name: create_message_rules,
        path: "/mailFolders/{{RID}}/messageRules",
        body: true
    );
    get!(
        doc: "List rules",
        name: list_message_rules,
        path: "/mailFolders/{{RID}}/messageRules"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_message_rules_count,
        path: "/mailFolders/{{RID}}/messageRules/$count"
    );
    delete!(
        doc: "Delete navigation property messageRules for users",
        name: delete_message_rules,
        path: "/mailFolders/{{RID}}/messageRules/{{id}}",
        params: message_rule_id
    );
    get!(
        doc: "Get messageRules from users",
        name: get_message_rules,
        path: "/mailFolders/{{RID}}/messageRules/{{id}}",
        params: message_rule_id
    );
    patch!(
        doc: "Update the navigation property messageRules in users",
        name: update_message_rules,
        path: "/mailFolders/{{RID}}/messageRules/{{id}}",
        body: true,
        params: message_rule_id
    );
    post!(
        doc: "Invoke action move",
        name: move_mail_folder,
        path: "/mailFolders/{{RID}}/move",
        body: true
    );
}
