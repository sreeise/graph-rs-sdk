use crate::client::Graph;
use crate::http::{GraphResponse, ResponseClient};
use crate::types::collection::Collection;
use crate::url::{UrlOrdering, FormatOrd};
use graph_rs_types::entitytypes::Calendar;
use reqwest::Method;
use std::marker::PhantomData;

fn ord_vec(last: &str) -> Vec<FormatOrd> {
    vec![FormatOrd::Insert(UrlOrdering::Last(last.into()))]
}

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

    get!(list, Collection<Calendar>, ord_vec("calendars"), true);
    get!(get, Calendar, ord_vec("calendar"), true);
    patch!(update, Calendar, ord_vec("calendars"), true, ());
    post!(create, Calendar, ord_vec("calendars"), true, ());
    delete!(delete, GraphResponse<()>, ord_vec("calendars"), true);
}
