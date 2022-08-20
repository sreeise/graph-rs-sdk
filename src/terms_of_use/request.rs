// GENERATED CODE

use crate::agreement_acceptances::{AgreementAcceptancesIdRequest, AgreementAcceptancesRequest};
use crate::agreements::{AgreementsIdRequest, AgreementsRequest};
use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(TermsOfUseRequest,);

impl<'a, Client> TermsOfUseRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn agreement_acceptances(&self) -> AgreementAcceptancesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client
            .set_ident(ResourceIdentity::AgreementAcceptances);
        AgreementAcceptancesRequest::new(self.client)
    }

    pub fn agreement_acceptance<ID: AsRef<str>>(
        &self,
        id: ID,
    ) -> AgreementAcceptancesIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client
            .set_ident(ResourceIdentity::AgreementAcceptances);
        AgreementAcceptancesIdRequest::new(id.as_ref(), self.client)
    }

    pub fn agreements(&self) -> AgreementsRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Agreements);
        AgreementsRequest::new(self.client)
    }

    pub fn agreement<ID: AsRef<str>>(&self, id: ID) -> AgreementsIdRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::Agreements);
        AgreementsIdRequest::new(id.as_ref(), self.client)
    }

    delete!({
        doc: "Delete navigation property termsOfUse for identityGovernance",
        name: delete_terms_of_use,
        response: NoContent,
        path: "/termsOfUse",
        has_body: false
    });
    get!({
        doc: "Get termsOfUse from identityGovernance",
        name: get_terms_of_use,
        response: serde_json::Value,
        path: "/termsOfUse",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property termsOfUse in identityGovernance",
        name: update_terms_of_use,
        response: NoContent,
        path: "/termsOfUse",
        has_body: true
    });
}
