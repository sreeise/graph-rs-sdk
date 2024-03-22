// GENERATED CODE

use crate::api_default_imports::*;

api_client!(
    InvitationsApiClient,
    InvitationsIdApiClient,
    ResourceIdentity::Invitations
);

impl InvitationsApiClient {
    post!(
        doc: "Create invitation",
        name: create_invitation,
        path: "/invitations",
        body: true
    );
    get!(
        doc: "Get entities from invitations",
        name: list_invitation,
        path: "/invitations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_invitations_count,
        path: "/invitations/$count"
    );
}

impl InvitationsIdApiClient {
    delete!(
        doc: "Delete entity from invitations",
        name: delete_invitation,
        path: "/invitations/{{RID}}"
    );
    get!(
        doc: "Get entity from invitations by key",
        name: get_invitation,
        path: "/invitations/{{RID}}"
    );
    patch!(
        doc: "Update entity in invitations",
        name: update_invitation,
        path: "/invitations/{{RID}}",
        body: true
    );
    get!(
        doc: "Get invitedUser from invitations",
        name: get_invited_user,
        path: "/invitations/{{RID}}/invitedUser"
    );
}
