use crate::client::{Graph, Ident};
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

    pub fn list(&self) -> IntoResponse<'a, I, Collection<User>> {
        self.client
            .request()
            .set_method(Method::GET)
            .remove(UrlOrdering::Ident(Ident::Me))
            .insert(UrlOrdering::ItemPath("users".into()))
            .format_ord();
        IntoResponse::new(self.client)
    }

    pub fn get(&self) -> ResponseClient<'a, I, User> {
        if self.client.ident().ne(&Ident::Me) {
            self.client
                .request()
                .remove(UrlOrdering::Ident(Ident::Drives))
                .insert(UrlOrdering::ItemPath("users".into()));
        }
        self.client.request().set_method(Method::GET).format_ord();
        ResponseClient::new(self.client)
    }

    pub fn create<T: serde::Serialize>(&self, user: &T) -> IntoResponse<'a, I, User> {
        self.client
            .request()
            .set_body(serde_json::to_string(user).unwrap())
            .set_method(Method::POST)
            .remove(UrlOrdering::Ident(Ident::Me))
            .insert(UrlOrdering::ItemPath("users".into()))
            .format_ord();
        IntoResponse::new(self.client)
    }

    pub fn update<T: serde::Serialize>(
        &self,
        user: &T,
    ) -> ResponseClient<'a, I, GraphResponse<()>> {
        self.client
            .request()
            .set_body(serde_json::to_string(user).unwrap())
            .set_method(Method::PATCH)
            .remove(UrlOrdering::Ident(Ident::Me))
            .insert(UrlOrdering::ItemPath("users".into()))
            .format_ord();
        ResponseClient::new(self.client)
    }

    pub fn delete(&self) -> ResponseClient<'a, I, GraphResponse<()>> {
        self.client
            .request()
            .set_method(Method::DELETE)
            .remove(UrlOrdering::Ident(Ident::Me))
            .insert(UrlOrdering::ItemPath("users".into()))
            .format_ord();
        ResponseClient::new(self.client)
    }
}
