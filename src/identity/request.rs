use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::{GraphResponse, IntoResponse};
use reqwest::Method;

register_client!(IdentityRequest,);
register_client!(IdentityContainerRequest,);
register_client!(IdentityProviderRequest,);
register_client!(ConditionalAccessRequest,);

#[allow(dead_code)]
impl<'a, Client> ConditionalAccessRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get policies from identity",
        name: get_policies,
        response: serde_json::Value,
        path: "/identity/conditionalAccess/policies/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property policies in identity",
        name: update_policies,
        response: GraphResponse<Content>,
        path: "/identity/conditionalAccess/policies/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property policies for identity",
        name: delete_policies,
        response: GraphResponse<Content>,
        path: "/identity/conditionalAccess/policies/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get policies from identity",
        name: list_policies,
        response: Collection<serde_json::Value>,
        path: "/identity/conditionalAccess/policies",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to policies for identity",
        name: create_policies,
        response: serde_json::Value,
        path: "/identity/conditionalAccess/policies",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get namedLocations from identity",
        name: get_named_locations,
        response: serde_json::Value,
        path: "/identity/conditionalAccess/namedLocations/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property namedLocations in identity",
        name: update_named_locations,
        response: GraphResponse<Content>,
        path: "/identity/conditionalAccess/namedLocations/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property namedLocations for identity",
        name: delete_named_locations,
        response: GraphResponse<Content>,
        path: "/identity/conditionalAccess/namedLocations/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get namedLocations from identity",
        name: list_named_locations,
        response: Collection<serde_json::Value>,
        path: "/identity/conditionalAccess/namedLocations",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Create new navigation property to namedLocations for identity",
        name: create_named_locations,
        response: serde_json::Value,
        path: "/identity/conditionalAccess/namedLocations",
        params: 0,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> IdentityProviderRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entities from identityProviders",
        name: list_identity_provider,
        response: Collection<serde_json::Value>,
        path: "/identityProviders",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Add new entity to identityProviders",
        name: create_identity_provider,
        response: serde_json::Value,
        path: "/identityProviders",
        params: 0,
        has_body: true
    });
    get!({
        doc: "# Get entity from identityProviders by key",
        name: get_identity_provider,
        response: serde_json::Value,
        path: "/identityProviders/{{id}}",
        params: 1,
        has_body: false
    });
    patch!({
        doc: "# Update entity in identityProviders",
        name: update_identity_provider,
        response: GraphResponse<Content>,
        path: "/identityProviders/{{id}}",
        params: 1,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from identityProviders",
        name: delete_identity_provider,
        response: GraphResponse<Content>,
        path: "/identityProviders/{{id}}",
        params: 1,
        has_body: false
    });
}

#[allow(dead_code)]
impl<'a, Client> IdentityContainerRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get identity",
        name: get_identity_container,
        response: serde_json::Value,
        path: "/identity",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update identity",
        name: update_identity_container,
        response: GraphResponse<Content>,
        path: "/identity",
        params: 0,
        has_body: true
    });
}

#[allow(dead_code)]
impl<'a, Client> IdentityRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn conditional_access(&self) -> ConditionalAccessRequest<'a, Client> {
        ConditionalAccessRequest::new(&self.client)
    }
    pub fn identity_container(&self) -> IdentityContainerRequest<'a, Client> {
        IdentityContainerRequest::new(&self.client)
    }
    pub fn identity_provider(&self) -> IdentityProviderRequest<'a, Client> {
        IdentityProviderRequest::new(&self.client)
    }
    get!({
        doc: "# Get conditionalAccess from identity",
        name: get_conditional_access,
        response: serde_json::Value,
        path: "/identity/conditionalAccess",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update the navigation property conditionalAccess in identity",
        name: update_conditional_access,
        response: GraphResponse<Content>,
        path: "/identity/conditionalAccess",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete navigation property conditionalAccess for identity",
        name: delete_conditional_access,
        response: GraphResponse<Content>,
        path: "/identity/conditionalAccess",
        params: 0,
        has_body: false
    });
}
