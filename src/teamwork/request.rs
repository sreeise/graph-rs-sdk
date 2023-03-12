// GENERATED CODE

use crate::api_default_imports::*;
use crate::teamwork::*;

resource_api_client!(TeamworkApiClient, ResourceIdentity::Teamwork);

impl TeamworkApiClient {
    api_client_link_id!(deleted_team, DeletedTeamsIdApiClient);
    api_client_link!(deleted_teams, DeletedTeamsApiClient);

    get!(
        doc: "Get teamwork",
        name: get_teamwork,
        path: "/teamwork"
    );
    patch!(
        doc: "Update teamwork",
        name: update_teamwork,
        path: "/teamwork",
        body: true
    );
    post!(
        doc: "Invoke action sendActivityNotificationToRecipients",
        name: send_activity_notification_to_recipients,
        path: "/teamwork/sendActivityNotificationToRecipients",
        body: true
    );
    post!(
        doc: "Create workforceIntegration",
        name: create_workforce_integrations,
        path: "/teamwork/workforceIntegrations",
        body: true
    );
    get!(
        doc: "List workforceIntegrations",
        name: list_workforce_integrations,
        path: "/teamwork/workforceIntegrations"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_workforce_integrations_count,
        path: "/teamwork/workforceIntegrations/$count"
    );
    delete!(
        doc: "Delete navigation property workforceIntegrations for teamwork",
        name: delete_workforce_integrations,
        path: "/teamwork/workforceIntegrations/{{id}}",
        params: workforce_integration_id
    );
    get!(
        doc: "Get workforceIntegrations from teamwork",
        name: get_workforce_integrations,
        path: "/teamwork/workforceIntegrations/{{id}}",
        params: workforce_integration_id
    );
    patch!(
        doc: "Update the navigation property workforceIntegrations in teamwork",
        name: update_workforce_integrations,
        path: "/teamwork/workforceIntegrations/{{id}}",
        body: true,
        params: workforce_integration_id
    );
}
