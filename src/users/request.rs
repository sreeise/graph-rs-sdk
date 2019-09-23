use crate::client::{Graph, Ident, IdentifyCommon, IdentifyMe};
use crate::http::{GraphResponse, IntoResponse, ResponseClient};
use crate::types::collection::Collection;
use crate::url::UrlOrdering;
use graph_rs_types::entitytypes::User;
use reqwest::Method;
use std::marker::PhantomData;

pub struct UserRequest<'a, I> {
    client: &'a Graph,
    ident: PhantomData<I>,
}

impl<'a, I> UserRequest<'a, I> {
    pub fn new(client: &'a Graph) -> UserRequest<'a, I> {
        UserRequest {
            client,
            ident: PhantomData,
        }
    }

    fn update_ord(&self, method: Method) -> &Self {
        self.client
            .request()
            .insert(UrlOrdering::ItemPath("users".into()))
            .remove(UrlOrdering::Ident(Ident::Me))
            .set_method(method)
            .format_ord();
        self
    }

    pub fn list(&self) -> IntoResponse<'a, I, Collection<User>> {
        self.update_ord(Method::GET);
        IntoResponse::new(self.client)
    }

    pub fn create<T: serde::Serialize>(&self, user: &T) -> IntoResponse<'a, I, User> {
        self.update_ord(Method::POST);
        self.client
            .request()
            .set_body(serde_json::to_string(user).unwrap());
        IntoResponse::new(self.client)
    }

    pub fn update<T: serde::Serialize>(
        &self,
        user: &T,
    ) -> ResponseClient<'a, I, GraphResponse<()>> {
        self.update_ord(Method::PATCH);
        self.client
            .request()
            .set_body(serde_json::to_string(user).unwrap());
        ResponseClient::new(self.client)
    }

    pub fn delete(&self) -> ResponseClient<'a, I, GraphResponse<()>> {
        self.update_ord(Method::DELETE);
        ResponseClient::new(self.client)
    }
}

impl<'a> UserRequest<'a, IdentifyMe> {
    pub fn get(&self) -> ResponseClient<'a, IdentifyMe, User> {
        self.client.request().set_method(Method::GET);
        ResponseClient::new(self.client)
    }
}

impl<'a> UserRequest<'a, IdentifyCommon> {
    pub fn get(&self) -> ResponseClient<'a, IdentifyCommon, User> {
        self.update_ord(Method::GET);
        ResponseClient::new(self.client)
    }
}
