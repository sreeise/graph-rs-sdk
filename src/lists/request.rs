use crate::client::*;
use crate::http::ResponseClient;
use crate::url::UrlOrdering;
use graph_rs_types::entitytypes::ItemActivity;
use reqwest::Method;
use std::marker::PhantomData;

client_struct!(ListRequest);

impl<'a, I> ListRequest<'a, I> {
    pub fn item_activity(&self, list_id: &str) -> ResponseClient<'a, I, ItemActivity> {
        self.client
            .request()
            .insert(UrlOrdering::ItemPath("lists".into()))
            .insert(UrlOrdering::Last(format!("{}/activities", list_id)))
            .set_method(Method::GET)
            .format_ord();
        ResponseClient::new(self.client)
    }
}
