// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    Oauth2PermissionGrantsApiClient,
    Oauth2PermissionGrantsIdApiClient,
    ResourceIdentity::Oauth2PermissionGrants
);

impl Oauth2PermissionGrantsApiClient {
    post!(
        doc: "Create oAuth2PermissionGrant (a delegated permission grant)",
        name: create_oauth2_permission_grant,
        path: "/oauth2PermissionGrants",
        body: true
    );
    get!(
        doc: "List oauth2PermissionGrants (delegated permission grants)",
        name: list_oauth2_permission_grant,
        path: "/oauth2PermissionGrants"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_oauth_2_permission_grants_count,
        path: "/oauth2PermissionGrants/$count"
    );
    get!(
        doc: "Invoke function delta",
        name: delta,
        path: "/oauth2PermissionGrants/delta()"
    );
}

impl Oauth2PermissionGrantsIdApiClient {
    delete!(
        doc: "Delete oAuth2PermissionGrant (a delegated permission grant)",
        name: delete_oauth2_permission_grant,
        path: "/oauth2PermissionGrants/{{RID}}"
    );
    get!(
        doc: "Get oAuth2PermissionGrant (a delegated permission grant)",
        name: get_oauth2_permission_grant,
        path: "/oauth2PermissionGrants/{{RID}}"
    );
    patch!(
        doc: "Update a delegated permission grant (oAuth2PermissionGrant)",
        name: update_oauth2_permission_grant,
        path: "/oauth2PermissionGrants/{{RID}}",
        body: true
    );
}
