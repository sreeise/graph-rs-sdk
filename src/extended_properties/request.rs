// NOT GENERATED CODE.

use crate::api_default_imports::*;

api_client!(
    ExtendedPropertiesApiClient,
    ResourceIdentity::ExtendedProperties
);

impl ExtendedPropertiesApiClient {
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
        name: get_single_value_extended_properties_count,
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
