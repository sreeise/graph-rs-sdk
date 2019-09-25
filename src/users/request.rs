use crate::client::{Graph, Ident, IdentifyCommon, IdentifyMe};
use crate::http::{GraphResponse, ResponseClient};
use crate::types::collection::Collection;
use crate::url::FormatOrd;
use crate::url::UrlOrdering;
use graph_rs_types::entitytypes::User;
use reqwest::Method;
use std::marker::PhantomData;

fn ord_vec() -> Vec<FormatOrd> {
    vec![
        FormatOrd::Insert(UrlOrdering::ItemPath("users".into())),
        FormatOrd::Remove(UrlOrdering::Ident(Ident::Me)),
    ]
}

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

    get!(list, Collection<User>, ord_vec(), true);
    post!(create, User, ord_vec(), true, ());
    patch!(update, GraphResponse<()>, ord_vec(), true, ());
    delete!(delete, GraphResponse<()>, ord_vec(), true);
}

impl<'a> UserRequest<'a, IdentifyMe> {
    request_method_ident!(get, IdentifyCommon, User, Vec::new(), Method::GET, true);
}

impl<'a> UserRequest<'a, IdentifyCommon> {
    request_method_ident!(get, IdentifyCommon, User, ord_vec(), Method::GET, true);
}
