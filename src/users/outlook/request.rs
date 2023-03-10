// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(OutlookApiClient, ResourceIdentity::Outlook);

impl OutlookApiClient {
    get!(
        doc: "Get outlook from users",
        name: get_outlook,
        path: "/outlook"
    );
    post!(
        doc: "Create Outlook category",
        name: create_master_categories,
        path: "/outlook/masterCategories",
        body: true
    );
    get!(
        doc: "List Outlook categories",
        name: list_master_categories,
        path: "/outlook/masterCategories"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_master_categories_count,
        path: "/outlook/masterCategories/$count"
    );
    delete!(
        doc: "Delete navigation property masterCategories for users",
        name: delete_master_categories,
        path: "/outlook/masterCategories/{{id}}",
        params: outlook_category_id
    );
    get!(
        doc: "Get masterCategories from users",
        name: get_master_categories,
        path: "/outlook/masterCategories/{{id}}",
        params: outlook_category_id
    );
    patch!(
        doc: "Update the navigation property masterCategories in users",
        name: update_master_categories,
        path: "/outlook/masterCategories/{{id}}",
        body: true,
        params: outlook_category_id
    );
    get!(
        doc: "Invoke function supportedLanguages",
        name: supported_languages,
        path: "/outlook/supportedLanguages()"
    );
    get!(
        doc: "Invoke function supportedTimeZones",
        name: outlook,
        path: "/outlook/supportedTimeZones(TimeZoneStandard='{{id}}')",
        params: time_zone_standard
    );
}
