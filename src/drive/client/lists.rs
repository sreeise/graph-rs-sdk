use crate::client::*;
use crate::drive::client::ResponseClient;
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
    pub fn activities(&self, list_id: &str) -> ResponseClient<'a, I, ItemActivity> {
        self.client
            .insert_ord(UrlOrdering::ItemPath("lists".into()))
            .insert_ord(UrlOrdering::Last(format!("{}/activities", list_id)));
        ResponseClient::new(self.client)
    }
}
