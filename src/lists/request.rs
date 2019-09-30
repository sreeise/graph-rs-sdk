use crate::client::*;
use crate::http::{GraphResponse, IntoResponse};
use graph_rs_types::entitytypes::ItemActivity;
use graph_rs_types::entitytypes::List;
use reqwest::Method;
use std::marker::PhantomData;

register_client!(ListRequest,);

impl<'a, I> ListRequest<'a, I> {
    get!( | get, List => "lists/{{id}}" );
    get!( | activity, ItemActivity => "lists/{{id}}/activities" );
    get!(
        || item_activity,
        ItemActivity =>
        "lists/{{id}}/activities/{{id2}}"
    );
    post!( [ create_item, ItemActivity => "lists/{{id}}/items" ] );
    delete!( [ delete_item, GraphResponse<()> => "lists/{{id}}/items" ] );
    patch!( [ || update_item, GraphResponse<()> => "lists/{{id}}/items/{{id2}}/fields" ] );
}
