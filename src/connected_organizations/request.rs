// GENERATED CODE

use crate::api_default_imports::*;
use graph_http::types::NoContent;

register_client!(ConnectedOrganizationsRequest,);
register_client!(ConnectedOrganizationsIdRequest, ());

impl<'a, Client> ConnectedOrganizationsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    post!({
        doc: "Create new navigation property to connectedOrganizations for identityGovernance",
        name: create_connected_organizations,
        response: serde_json::Value,
        path: "/connectedOrganizations",
        has_body: true
    });
    get!({
        doc: "Get connectedOrganizations from identityGovernance",
        name: list_connected_organizations,
        response: serde_json::Value,
        path: "/connectedOrganizations",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: count,
        response: serde_json::Value,
        path: "/connectedOrganizations/$count",
        has_body: false
    });
}

impl<'a, Client> ConnectedOrganizationsIdRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    delete!({
        doc: "Delete navigation property connectedOrganizations for identityGovernance",
        name: delete_connected_organizations,
        response: NoContent,
        path: "/connectedOrganizations/{{RID}}",
        has_body: false
    });
    get!({
        doc: "Get connectedOrganizations from identityGovernance",
        name: get_connected_organizations,
        response: serde_json::Value,
        path: "/connectedOrganizations/{{RID}}",
        has_body: false
    });
    patch!({
        doc: "Update the navigation property connectedOrganizations in identityGovernance",
        name: update_connected_organizations,
        response: NoContent,
        path: "/connectedOrganizations/{{RID}}",
        has_body: true
    });
    post!({
        doc: "Create new navigation property to externalSponsors for identityGovernance",
        name: create_external_sponsors,
        response: serde_json::Value,
        path: "/connectedOrganizations/{{RID}}/externalSponsors",
        has_body: true
    });
    get!({
        doc: "Get externalSponsors from identityGovernance",
        name: list_external_sponsors,
        response: serde_json::Value,
        path: "/connectedOrganizations/{{RID}}/externalSponsors",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_external_sponsors_count,
        response: serde_json::Value,
        path: "/connectedOrganizations/{{RID}}/externalSponsors/$count",
        has_body: false
    });
    post!({
        doc: "Create new navigation property ref to externalSponsors for identityGovernance",
        name: create_ref_external_sponsors,
        response: NoContent,
        path: "/connectedOrganizations/{{RID}}/externalSponsors/$ref",
        has_body: true
    });
    get!({
        doc: "Get ref of externalSponsors from identityGovernance",
        name: list_ref_external_sponsors,
        response: serde_json::Value,
        path: "/connectedOrganizations/{{RID}}/externalSponsors/$ref",
        has_body: false
    });
    post!({
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_available_extension_properties,
        response: serde_json::Value,
        path: "/connectedOrganizations/{{RID}}/externalSponsors/microsoft.graph.getAvailableExtensionProperties",
        has_body: true
    });
    post!({
        doc: "Invoke action getByIds",
        name: get_external_sponsors_by_ids,
        response: serde_json::Value,
        path: "/connectedOrganizations/{{RID}}/externalSponsors/microsoft.graph.getByIds",
        has_body: true
    });
    post!({
        doc: "Invoke action validateProperties",
        name: validate_external_sponsors_properties,
        response: NoContent,
        path: "/connectedOrganizations/{{RID}}/externalSponsors/microsoft.graph.validateProperties",
        has_body: true
    });
    delete!({
        doc: "Delete ref of navigation property externalSponsors for identityGovernance",
        name: delete_ref_external_sponsors,
        response: NoContent,
        path: "/connectedOrganizations/{{RID}}/externalSponsors/{{id}}/$ref",
        params: [ directory_object_id ],
        has_body: false
    });
    post!({
        doc: "Create new navigation property to internalSponsors for identityGovernance",
        name: create_internal_sponsors,
        response: serde_json::Value,
        path: "/connectedOrganizations/{{RID}}/internalSponsors",
        has_body: true
    });
    get!({
        doc: "Get internalSponsors from identityGovernance",
        name: list_internal_sponsors,
        response: serde_json::Value,
        path: "/connectedOrganizations/{{RID}}/internalSponsors",
        has_body: false
    });
    get!({
        doc: "Get the number of the resource",
        name: get_internal_sponsors_count,
        response: serde_json::Value,
        path: "/connectedOrganizations/{{RID}}/internalSponsors/$count",
        has_body: false
    });
    post!({
        doc: "Create new navigation property ref to internalSponsors for identityGovernance",
        name: create_ref_internal_sponsors,
        response: NoContent,
        path: "/connectedOrganizations/{{RID}}/internalSponsors/$ref",
        has_body: true
    });
    get!({
        doc: "Get ref of internalSponsors from identityGovernance",
        name: list_ref_internal_sponsors,
        response: serde_json::Value,
        path: "/connectedOrganizations/{{RID}}/internalSponsors/$ref",
        has_body: false
    });
    post!({
        doc: "Invoke action getAvailableExtensionProperties",
        name: get_internal_sponsors_available_extension_properties,
        response: serde_json::Value,
        path: "/connectedOrganizations/{{RID}}/internalSponsors/microsoft.graph.getAvailableExtensionProperties",
        has_body: true
    });
    post!({
        doc: "Invoke action getByIds",
        name: get_internal_sponsors_by_ids,
        response: serde_json::Value,
        path: "/connectedOrganizations/{{RID}}/internalSponsors/microsoft.graph.getByIds",
        has_body: true
    });
    post!({
        doc: "Invoke action validateProperties",
        name: validate_internal_sponsors_properties,
        response: NoContent,
        path: "/connectedOrganizations/{{RID}}/internalSponsors/microsoft.graph.validateProperties",
        has_body: true
    });
    delete!({
        doc: "Delete ref of navigation property internalSponsors for identityGovernance",
        name: delete_ref_internal_sponsors,
        response: NoContent,
        path: "/connectedOrganizations/{{RID}}/internalSponsors/{{id}}/$ref",
        params: [ directory_object_id ],
        has_body: false
    });
}
