// GENERATED CODE

use crate::api_default_imports::*;

resource_api_client!(
    TeamsTemplatesApiClient,
    TeamsTemplatesIdApiClient,
    ResourceIdentity::TeamsTemplates
);

impl TeamsTemplatesApiClient {
    post!(
        doc: "Add new entity to teamsTemplates",
        name: create_teams_template,
        path: "/teamsTemplates",
        body: true
    );
    get!(
        doc: "Get entities from teamsTemplates",
        name: list_teams_template,
        path: "/teamsTemplates"
    );
    get!(
        doc: "Get the number of the resource",
        name: get_teams_templates_count,
        path: "/teamsTemplates/$count"
    );
}

impl TeamsTemplatesIdApiClient {
    delete!(
        doc: "Delete entity from teamsTemplates",
        name: delete_teams_template,
        path: "/teamsTemplates/{{RID}}"
    );
    get!(
        doc: "Get entity from teamsTemplates by key",
        name: get_teams_template,
        path: "/teamsTemplates/{{RID}}"
    );
    patch!(
        doc: "Update entity in teamsTemplates",
        name: update_teams_template,
        path: "/teamsTemplates/{{RID}}",
        body: true
    );
}
