use crate::client::*;
use crate::http::ResponseClient;
use crate::url::UrlOrdering;
use graph_rs_types::entitytypes::ItemActivity;
use std::marker::PhantomData;

pub struct ListRequest<'a, I> {
    client: &'a Graph,
    ident: PhantomData<I>,
}

impl<'a, I> ListRequest<'a, I> {
    pub fn new(client: &'a Graph) -> ListRequest<'a, I> {
        ListRequest {
            client,
            ident: PhantomData,
        }
    }
}

impl<'a, I> ListRequest<'a, I> {
    pub fn item_activity(&self, list_id: &str) -> ResponseClient<'a, I, ItemActivity> {
        self.client
            .request()
            .insert(UrlOrdering::ItemPath("lists".into()))
            .insert(UrlOrdering::Last(format!("{}/activities", list_id)))
            .format_ord();
        ResponseClient::new(self.client)
    }
}
