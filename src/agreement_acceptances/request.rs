// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    AgreementAcceptancesApiClient,
    AgreementAcceptancesIdApiClient,
    ResourceIdentity::AgreementAcceptances
);

impl AgreementAcceptancesApiClient {
    post!(
        doc: "Add new entity to agreementAcceptances",
        name: create_agreement_acceptance,
        path: "/agreementAcceptances",
        body: true
    );
    get!(
        doc: "Get entities from agreementAcceptances",
        name: list_agreement_acceptance,
        path: "/agreementAcceptances"
    );
}

impl AgreementAcceptancesIdApiClient {
    delete!(
        doc: "Delete entity from agreementAcceptances",
        name: delete_agreement_acceptance,
        path: "/agreementAcceptances/{{RID}}"
    );
    get!(
        doc: "Get entity from agreementAcceptances by key",
        name: get_agreement_acceptance,
        path: "/agreementAcceptances/{{RID}}"
    );
    patch!(
        doc: "Update entity in agreementAcceptances",
        name: update_agreement_acceptance,
        path: "/agreementAcceptances/{{RID}}",
        body: true
    );
}
