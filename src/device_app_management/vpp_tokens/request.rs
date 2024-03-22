// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    VppTokensApiClient,
    VppTokensIdApiClient,
    ResourceIdentity::VppTokens
);

impl VppTokensApiClient {
    post!(
        doc: "Create new navigation property to vppTokens for deviceAppManagement",
        name: create_vpp_tokens,
        path: "/vppTokens",
        body: true
    );
    get!(
        doc: "Get vppTokens from deviceAppManagement",
        name: list_vpp_tokens,
        path: "/vppTokens"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_vpp_tokens_count,
        path: "/vppTokens/$count"
    );
}

impl VppTokensIdApiClient {
    delete!(
        doc: "Delete navigation property vppTokens for deviceAppManagement",
        name: delete_vpp_tokens,
        path: "/vppTokens/{{RID}}"
    );
    get!(
        doc: "Get vppTokens from deviceAppManagement",
        name: get_vpp_tokens,
        path: "/vppTokens/{{RID}}"
    );
    patch!(
        doc: "Update the navigation property vppTokens in deviceAppManagement",
        name: update_vpp_tokens,
        path: "/vppTokens/{{RID}}",
        body: true
    );
    post!(
        doc: "Invoke action syncLicenses",
        name: sync_licenses,
        path: "/vppTokens/{{RID}}/syncLicenses"
    );
}
