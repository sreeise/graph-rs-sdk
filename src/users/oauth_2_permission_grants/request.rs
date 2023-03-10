// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    Oauth2PermissionGrantsApiClient,
    Oauth2PermissionGrantsIdApiClient,
    ResourceIdentity::Oauth2PermissionGrants
);

impl Oauth2PermissionGrantsApiClient {
    get!(
        doc: "List oauth2PermissionGrants",
        name: users_list_oauth_2_permission_grants,
        path: "/oauth2PermissionGrants"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_oauth_2_permission_grants_count,
        path: "/oauth2PermissionGrants/$count"
    );
}

impl Oauth2PermissionGrantsIdApiClient {
    get!(
        doc: "Get oauth2PermissionGrants from users",
        name: users_get_oauth_2_permission_grants,
        path: "/oauth2PermissionGrants/{{RID}}"
    );
}
