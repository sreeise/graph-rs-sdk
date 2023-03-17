// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    AgreementsApiClient,
    AgreementsIdApiClient,
    ResourceIdentity::Agreements
);

impl AgreementsApiClient {
    post!(
        doc: "Add new entity to agreements",
        name: create_agreement,
        path: "/agreements",
        body: true
    );
    get!(
        doc: "Get entities from agreements",
        name: list_agreement,
        path: "/agreements"
    );
}

impl AgreementsIdApiClient {
    delete!(
        doc: "Delete entity from agreements",
        name: delete_agreement,
        path: "/agreements/{{RID}}"
    );
    get!(
        doc: "Get entity from agreements by key",
        name: get_agreement,
        path: "/agreements/{{RID}}"
    );
    patch!(
        doc: "Update entity in agreements",
        name: update_agreement,
        path: "/agreements/{{RID}}",
        body: true
    );
    post!(
        doc: "Create new navigation property to acceptances for agreements",
        name: create_acceptances,
        path: "/agreements/{{RID}}/acceptances",
        body: true
    );
    get!(
        doc: "List acceptances",
        name: list_acceptances,
        path: "/agreements/{{RID}}/acceptances"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_acceptances_count,
        path: "/agreements/{{RID}}/acceptances/$count"
    );
    delete!(
        doc: "Delete navigation property acceptances for agreements",
        name: delete_acceptances,
        path: "/agreements/{{RID}}/acceptances/{{id}}",
        params: agreement_acceptance_id
    );
    get!(
        doc: "Get acceptances from agreements",
        name: get_acceptances,
        path: "/agreements/{{RID}}/acceptances/{{id}}",
        params: agreement_acceptance_id
    );
    patch!(
        doc: "Update the navigation property acceptances in agreements",
        name: update_acceptances,
        path: "/agreements/{{RID}}/acceptances/{{id}}",
        body: true,
        params: agreement_acceptance_id
    );
}
