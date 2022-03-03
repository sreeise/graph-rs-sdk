// GENERATED CODE

use crate::api_default_imports::*;
use crate::service_announcement::ServiceAnnouncementRequest;
use graph_http::types::NoContent;

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
