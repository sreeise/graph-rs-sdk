use crate::client::Graph;
use crate::http::{GraphResponse, ResponseClient};
use crate::types::collection::Collection;
use crate::url::{FormatOrd, UrlOrdering};
use graph_rs_types::entitytypes::Calendar;
use reqwest::Method;
use std::marker::PhantomData;

fn ord_vec(last: &str) -> Vec<FormatOrd> {
    vec![FormatOrd::Insert(UrlOrdering::Last(last.into()))]
}

client_struct!(CalendarRequest);

impl<'a, I> CalendarRequest<'a, I> {
    get!(list, Collection<Calendar>, ord_vec("calendars"), true);
    get!(get, Calendar, ord_vec("calendar"), true);
    patch!(update, Calendar, ord_vec("calendars"), true, ());
    post!(create, Calendar, ord_vec("calendars"), true, ());
    delete!(delete, GraphResponse<()>, ord_vec("calendars"), true);
}
