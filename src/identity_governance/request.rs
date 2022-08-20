// GENERATED CODE

use crate::access_reviews::AccessReviewsRequest;
use crate::api_default_imports::*;
use crate::app_consent::AppConsentRequest;
use crate::entitlement_management::EntitlementManagementRequest;
use crate::terms_of_use::TermsOfUseRequest;
use graph_http::types::NoContent;

register_client!(IdentityGovernanceRequest,);

impl<'a, Client> IdentityGovernanceRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn access_reviews(&self) -> AccessReviewsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::AccessReviews);
        AccessReviewsRequest::new(self.client)
    }

    pub fn app_consent(&self) -> AppConsentRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::AppConsent);
        AppConsentRequest::new(self.client)
    }

    pub fn entitlement_management(&self) -> EntitlementManagementRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client
            .set_ident(ResourceIdentity::EntitlementManagement);
        EntitlementManagementRequest::new(self.client)
    }

    pub fn terms_of_use(&self) -> TermsOfUseRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::TermsOfUse);
        TermsOfUseRequest::new(self.client)
    }

    get!({
        doc: "Get identityGovernance",
        name: get_identity_governance,
        response: serde_json::Value,
        path: "/identityGovernance",
        has_body: false
    });
    patch!({
        doc: "Update identityGovernance",
        name: update_identity_governance,
        response: NoContent,
        path: "/identityGovernance",
        has_body: true
    });
}
