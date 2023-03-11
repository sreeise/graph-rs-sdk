// GENERATED CODE

use crate::api_default_imports::*;
use crate::extended_properties::*;
use crate::users::*;

resource_api_client!(
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
    post!(
        doc: "Create new navigation property to multiValueExtendedProperties for users",
        name: create_multi_value_extended_properties,
        path: "/mailFolders/{{RID}}/multiValueExtendedProperties",
        body: true
    );
    get!(
        doc: "Get multiValueExtendedProperties from users",
        name: list_multi_value_extended_properties,
        path: "/mailFolders/{{RID}}/multiValueExtendedProperties"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_multi_value_extended_properties_count,
        path: "/mailFolders/{{RID}}/multiValueExtendedProperties/$count"
    );
    delete!(
        doc: "Delete navigation property multiValueExtendedProperties for users",
        name: delete_multi_value_extended_properties,
        path: "/mailFolders/{{RID}}/multiValueExtendedProperties/{{id}}",
        params: multi_value_legacy_extended_property_id
    );
    get!(
        doc: "Get multiValueExtendedProperties from users",
        name: get_multi_value_extended_properties,
        path: "/mailFolders/{{RID}}/multiValueExtendedProperties/{{id}}",
        params: multi_value_legacy_extended_property_id
    );
    patch!(
        doc: "Update the navigation property multiValueExtendedProperties in users",
        name: update_multi_value_extended_properties,
        path: "/mailFolders/{{RID}}/multiValueExtendedProperties/{{id}}",
        body: true,
        params: multi_value_legacy_extended_property_id
    );
    post!(
        doc: "Create new navigation property to singleValueExtendedProperties for users",
        name: create_single_value_extended_properties,
        path: "/mailFolders/{{RID}}/singleValueExtendedProperties",
        body: true
    );
    get!(
        doc: "Get singleValueExtendedProperties from users",
        name: list_single_value_extended_properties,
        path: "/mailFolders/{{RID}}/singleValueExtendedProperties"
    );
    get!(
        doc: "Get the number of the resource",
        name: count,
        path: "/mailFolders/{{RID}}/singleValueExtendedProperties/$count"
    );
    delete!(
        doc: "Delete navigation property singleValueExtendedProperties for users",
        name: delete_single_value_extended_properties,
        path: "/mailFolders/{{RID}}/singleValueExtendedProperties/{{id}}",
        params: single_value_legacy_extended_property_id
    );
    get!(
        doc: "Get singleValueExtendedProperties from users",
        name: get_single_value_extended_properties,
        path: "/mailFolders/{{RID}}/singleValueExtendedProperties/{{id}}",
        params: single_value_legacy_extended_property_id
    );
    patch!(
        doc: "Update the navigation property singleValueExtendedProperties in users",
        name: update_single_value_extended_properties,
        path: "/mailFolders/{{RID}}/singleValueExtendedProperties/{{id}}",
        body: true,
        params: single_value_legacy_extended_property_id
    );
}
