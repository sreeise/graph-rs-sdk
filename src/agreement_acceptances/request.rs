// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(AgreementAcceptancesRequest,);
register_client!(AgreementAcceptancesIdRequest, ());

impl<'a, Client> AgreementAcceptancesRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "Get entities from agreementAcceptances",
        name: list_agreement_acceptance,
        response: serde_json::Value,
        path: "/agreementAcceptances",
        has_body: false
    });
    post!({
        doc: "Add new entity to agreementAcceptances",
        name: create_agreement_acceptance,
        response: serde_json::Value,
        path: "/agreementAcceptances",
        has_body: true
    });
}

impl<'a, Client> AgreementAcceptancesIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    patch!({
        doc: "Update entity in agreementAcceptances",
        name: update_agreement_acceptance,
        response: NoContent,
        path: "/agreementAcceptances/{{RID}}",
        has_body: true
    });
    delete!({
        doc: "Delete entity from agreementAcceptances",
        name: delete_agreement_acceptance,
        response: NoContent,
        path: "/agreementAcceptances/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get entity from agreementAcceptances by key",
        name: get_agreement_acceptance,
        response: serde_json::Value,
        path: "/agreementAcceptances/{{RID}}",
        has_body: false
    });
}
