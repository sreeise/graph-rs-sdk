use crate::client::Graph;
use crate::http::{GraphResponse, ResponseClient};
use crate::types::collection::Collection;
use crate::url::UrlOrdering;
use graph_rs_types::entitytypes::Calendar;
use reqwest::Method;
use std::marker::PhantomData;

pub struct CalendarRequest<'a, I> {
    client: &'a Graph,
    ident: PhantomData<I>,
}

impl<'a, I> CalendarRequest<'a, I> {
    pub fn new(client: &'a Graph) -> CalendarRequest<'a, I> {
        CalendarRequest {
            client,
            ident: PhantomData,
        }
    }

    pub fn list(&self) -> ResponseClient<'a, I, Collection<Calendar>> {
        self.client
            .request()
            .set_method(Method::GET)
            .insert(UrlOrdering::Last("calendars".into()))
            .format_ord();
        ResponseClient::new(self.client)
    }

    pub fn get(&self) -> ResponseClient<'a, I, Calendar> {
        self.client
            .request()
            .set_method(Method::GET)
            .insert(UrlOrdering::Last("calendar".into()))
            .format_ord();
        ResponseClient::new(self.client)
    }

    pub fn update<B: serde::Serialize>(&self, body: &B) -> ResponseClient<'a, I, Calendar> {
        self.client
            .request()
            .set_method(Method::PATCH)
            .insert(UrlOrdering::Last("calendars".into()))
            .set_body(serde_json::to_string_pretty(body).unwrap())
            .format_ord();
        ResponseClient::new(self.client)
    }

    pub fn create<B: serde::Serialize>(&self, body: &B) -> ResponseClient<'a, I, Calendar> {
        self.client
            .request()
            .set_method(Method::POST)
            .insert(UrlOrdering::Last("calendars".into()))
            .set_body(serde_json::to_string_pretty(body).unwrap())
            .format_ord();
        ResponseClient::new(self.client)
    }

    pub fn delete(&self) -> ResponseClient<'a, I, GraphResponse<()>> {
        self.client
            .request()
            .set_method(Method::DELETE)
            .insert(UrlOrdering::Last("calendars".into()))
            .format_ord();
        ResponseClient::new(self.client)
    }
}
