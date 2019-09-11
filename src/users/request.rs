use crate::client::{Graph, Ident, UrlOrdering};
use crate::http::{IntoResponse, ResponseClient};
use crate::types::collection::Collection;
use crate::types::statusresponse::StatusResponse;
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
            .set_method(Method::GET)
            .remove_ord(UrlOrdering::Ident(Ident::Me))
            .insert_ord(UrlOrdering::ItemPath("users".into()))
            .format_ord();
        IntoResponse::new(self.client)
    }

    pub fn get(&self) -> ResponseClient<'a, I, User> {
        if self.client.ident().ne(&Ident::Me) {
            self.client
                .remove_ord(UrlOrdering::Ident(Ident::Drives))
                .insert_ord(UrlOrdering::ItemPath("users".into()));
        }
        self.client.set_method(Method::GET).format_ord();
        ResponseClient::new(self.client)
    }

    pub fn create(&self, user: &User) -> IntoResponse<'a, I, User> {
        self.client
            .body(serde_json::to_string(user).unwrap())
            .set_method(Method::POST)
            .remove_ord(UrlOrdering::Ident(Ident::Me))
            .insert_ord(UrlOrdering::ItemPath("users".into()))
            .format_ord();
        IntoResponse::new(self.client)
    }

    pub fn update(&self, user: &User) -> ResponseClient<'a, I, StatusResponse> {
        self.client
            .body(serde_json::to_string(user).unwrap())
            .set_method(Method::PATCH)
            .remove_ord(UrlOrdering::Ident(Ident::Me))
            .insert_ord(UrlOrdering::ItemPath("users".into()))
            .format_ord();
        ResponseClient::new(self.client)
    }

    pub fn delete(&self) -> ResponseClient<'a, I, StatusResponse> {
        self.client
            .set_method(Method::DELETE)
            .remove_ord(UrlOrdering::Ident(Ident::Me))
            .insert_ord(UrlOrdering::ItemPath("users".into()))
            .format_ord();
        ResponseClient::new(self.client)
    }
}
