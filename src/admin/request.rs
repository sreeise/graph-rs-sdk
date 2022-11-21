// GENERATED CODE

use crate::api_default_imports::*;
use crate::service_announcement::ServiceAnnouncementRequest;
use graph_http::api_impl::*;
use graph_http::types::NoContent;
use graph_http::url::GraphUrl;

pub trait AdminClient {
    fn get_admin<ID: AsRef<str>>(&self, admin_id: ID) -> ResponseHandler;
}

resource_client!(AdminRequest2);

impl AdminRequest2 {
    /// Get Admin
    get!( name: get_admin, path: "/admin", params: [ admin_id, ] );

    get!( name: get_admins, path: "/admin" );

    get!( name: get_admin_id2, path: "/admin/{{RID}}/name/{{id}}/id/{{id1}}", body, params: [ name, id, ] );
}

/*
request! {
        get (get_admin, "/admin", [ admin_id ])
    }
impl ResourceApiClient for AdminRequest2 {
    fn url(&self) -> GraphUrl {
        self.resource_config.url.clone()
    }

    fn registry(&self) -> &Handlebars {
        &self.registry
    }
}

 */
register_client!(AdminRequest,);

impl<'a, Client> AdminRequest<'a, Client>
where
    Client: graph_http::RequestClient,
{
    pub fn service_announcement(&self) -> ServiceAnnouncementRequest<'a, Client> {
        self.client
            .request
            .extend_path(&[self.client.ident().as_ref()]);
        self.client.set_ident(ResourceIdentity::ServiceAnnouncement);
        ServiceAnnouncementRequest::new(self.client)
    }

    get!({
        doc: "Get admin",
        name: get_admin,
        response: serde_json::Value,
        path: "/admin",
        has_body: false
    });
    patch!({
        doc: "Update admin",
        name: update_admin,
        response: NoContent,
        path: "/admin",
        has_body: true
    });
}
