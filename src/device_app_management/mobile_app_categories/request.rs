// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    MobileAppCategoriesApiClient,
    MobileAppCategoriesIdApiClient,
    ResourceIdentity::MobileAppCategories
);

impl MobileAppCategoriesApiClient {
    post!(
        doc: "Create new navigation property to mobileAppCategories for deviceAppManagement",
        name: create_mobile_app_categories,
        path: "/mobileAppCategories",
        body: true
    );
    get!(
        doc: "Get mobileAppCategories from deviceAppManagement",
        name: list_mobile_app_categories,
        path: "/mobileAppCategories"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_mobile_app_categories_count,
        path: "/mobileAppCategories/$count"
    );
}

impl MobileAppCategoriesIdApiClient {
    delete!(
        doc: "Delete navigation property mobileAppCategories for deviceAppManagement",
        name: delete_mobile_app_categories,
        path: "/mobileAppCategories/{{RID}}"
    );
    get!(
        doc: "Get mobileAppCategories from deviceAppManagement",
        name: get_mobile_app_categories,
        path: "/mobileAppCategories/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property mobileAppCategories in deviceAppManagement",
        name: update_mobile_app_categories,
        path: "/mobileAppCategories/{{RID}}",
        body: true
    );
}
