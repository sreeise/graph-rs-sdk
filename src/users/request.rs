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

client_struct!(UserRequest);

impl<'a, I> UserRequest<'a, I> {
    get!(list, Collection<User>, ord_vec(), true);
    post!(create, User, ord_vec(), true, ());
    patch!(update, GraphResponse<()>, ord_vec(), true, ());
    delete!(delete, GraphResponse<()>, ord_vec(), true);
}

impl<'a> UserRequest<'a, IdentifyMe> {
    request_method_ident!(get, IdentifyCommon, User, vec![], Method::GET, true);
}

impl<'a> UserRequest<'a, IdentifyCommon> {
    request_method_ident!(get, IdentifyCommon, User, ord_vec(), Method::GET, true);
}
