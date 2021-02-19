// NOT GENERATED CODE

use crate::client::Graph;
use graph_http::types::Collection;
use graph_http::types::DeltaPhantom;
use graph_http::types::NoContent;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(OrgContactRequest,);
register_client!(OrgContactsRequest, ());

impl<'a, Client> OrgContactRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> OrgContactsRequest<'a, Client> {
        OrgContactsRequest::new(id.as_ref(), self.client)
    }
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/contacts/delta()",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get entities from contacts",
        name: list_org_contact,
        response: Collection<serde_json::Value>,
        path: "/contacts",
        params: 0,
        has_body: false
    });
    post!({
        doc: "# Add new entity to contacts",
        name: create_org_contact,
        response: serde_json::Value,
        path: "/contacts",
        params: 0,
        has_body: true
    });
}

impl<'a, Client> OrgContactsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    get!({
        doc: "# Get entity from contacts by key",
        name: get_org_contact,
        response: serde_json::Value,
        path: "/contacts/{{RID}}",
        params: 0,
        has_body: false
    });
    patch!({
        doc: "# Update entity in contacts",
        name: update_org_contact,
        response: NoContent,
        path: "/contacts/{{RID}}",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from contacts",
        name: delete_org_contact,
        response: NoContent,
        path: "/contacts/{{RID}}",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get directReports from contacts",
        name: list_direct_reports,
        response: Collection<serde_json::Value>,
        path: "/contacts/{{RID}}/directReports",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get directReports from contacts",
        name: get_direct_reports,
        response: serde_json::Value,
        path: "/contacts/{{RID}}/directReports/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get manager from contacts",
        name: get_manager,
        response: serde_json::Value,
        path: "/contacts/{{RID}}/manager",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get memberOf from contacts",
        name: list_member_of,
        response: Collection<serde_json::Value>,
        path: "/contacts/{{RID}}/memberOf",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get memberOf from contacts",
        name: get_member_of,
        response: serde_json::Value,
        path: "/contacts/{{RID}}/memberOf/{{id}}",
        params: 1,
        has_body: false
    });
    get!({
        doc: "# Get transitiveMemberOf from contacts",
        name: list_transitive_member_of,
        response: Collection<serde_json::Value>,
        path: "/contacts/{{RID}}/transitiveMemberOf",
        params: 0,
        has_body: false
    });
    get!({
        doc: "# Get transitiveMemberOf from contacts",
        name: get_transitive_member_of,
        response: serde_json::Value,
        path: "/contacts/{{RID}}/transitiveMemberOf/{{id}}",
        params: 1,
        has_body: false
    });
}
