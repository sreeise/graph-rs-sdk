// GENERATED CODE

use crate::api_default_imports::*;
use crate::identity_governance::{
    AccessPackagesApiClient, AccessPackagesIdApiClient, AccessReviewsApiClient,
    EntitlementManagementApiClient,
};

resource_api_client!(
    IdentityGovernanceApiClient,
    ResourceIdentity::IdentityGovernance
);

impl IdentityGovernanceApiClient {
    api_client_link!(
        access_reviews,
        ResourceIdentity::AccessReviews,
        AccessReviewsApiClient
    );
    api_client_link!(
        entitlement_management,
        ResourceIdentity::EntitlementManagement,
        EntitlementManagementApiClient
    );

    get!(
        doc: "Get identityGovernance",
        name: get_identity_governance,
        path: "/identityGovernance"
    );
    patch!(
        doc: "Update identityGovernance",
        name: update_identity_governance,
        path: "/identityGovernance",
        body: true
    );
}
