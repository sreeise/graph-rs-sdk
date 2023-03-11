// NOT GENERATED CODE.

use crate::api_default_imports::*;

resource_api_client!(
    ExtendedPropertiesApiClient,
    ResourceIdentity::ExtendedProperties
);

impl ExtendedPropertiesApiClient {
    post!(
        name: get_multi_value_extended_properties,
        path: "/multiValueExtendedProperties/{{id}}",
        params: id
    );

    get!(
        name: create_multi_value_extended_properties,
        path: "/multiValueExtendedProperties"
    );

    post!(
        name: get_single_value_extended_properties,
        path: "/singleValueExtendedProperties/{{id}}",
        params: id
    );

    get!(
        name: create_single_value_extended_properties,
        path: "/singleValueExtendedProperties"
    );

    get!(
        name: get_single_value_extended_properties_count,
        path: "multiValueExtendedProperties/$count"
    );

    patch!(
        doc: "Update the navigation property multiValueExtendedProperties",
        name: update_multi_value_extended_properties,
        path: "/contactFolders/{{RID}}/multiValueExtendedProperties/{{id}}",
        body: true,
        params: multi_value_legacy_extended_property_id
    );

    delete!(
        doc: "Delete navigation property singleValueExtendedProperties",
        name: delete_single_value_extended_properties,
        path: "/singleValueExtendedProperties/{{id}}",
        params: single_value_legacy_extended_property_id
    );

    delete!(
        doc: "Delete navigation property singleValueExtendedProperties",
        name: delete_multi_value_extended_properties,
        path: "/multiValueExtendedProperties/{{id}}",
        params: multi_value_legacy_extended_property_id
    );
}
