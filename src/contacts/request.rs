use crate::client::Graph;
use crate::core::ResourceIdentity;
use crate::extended_properties::ExtendedPropertiesRequest;
use graph_http::types::Collection;
use graph_http::types::Content;
use graph_http::types::DeltaPhantom;
use graph_http::GraphResponse;
use graph_http::IntoResponse;
use handlebars::*;
use reqwest::Method;

register_client!(ContactRequest,);
register_client!(ContactsRequest, ());
register_client!(OrgContactRequest,);

impl<'a, Client> ContactRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn id<ID: AsRef<str>>(&self, id: ID) -> ContactsRequest<'a, Client> {
        self.client.set_ident(ResourceIdentity::Contacts);
        ContactsRequest::new(id.as_ref(), self.client)
    }
    pub fn org_contact(&self) -> OrgContactRequest<'a, Client> {
        OrgContactRequest::new(self.client)
    }
    get!({
        doc: "# Invoke function delta",
        name: delta,
        response: DeltaPhantom<serde_json::Value>,
        path: "/contacts/delta()",
        params: 0,
        has_body: false
    });
}

impl<'a, Client> ContactsRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn extended_properties(&self) -> ExtendedPropertiesRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref(), self.id.as_str()]);
        self.client.set_ident(ResourceIdentity::ExtendedProperties);
        ExtendedPropertiesRequest::new(self.client)
    }
    pub fn org_contact(&self) -> OrgContactRequest<'a, Client> {
        OrgContactRequest::new(self.client)
    }
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

impl<'a, Client> OrgContactRequest<'a, Client>
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
        response: GraphResponse<Content>,
        path: "/contacts/{{RID}}",
        params: 0,
        has_body: true
    });
    delete!({
        doc: "# Delete entity from contacts",
        name: delete_org_contact,
        response: GraphResponse<Content>,
        path: "/contacts/{{RID}}",
        params: 0,
        has_body: false
    });
}
