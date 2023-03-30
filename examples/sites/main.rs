#![allow(dead_code)]

use graph_rs_sdk::error::GraphResult;

mod get_sites;
mod lists_items;

#[tokio::main]
async fn main() -> GraphResult<()> {
    get_sites::list_sites().await?;
    get_sites::get_site().await?;
    lists_items::get_list_item().await?;
    lists_items::list_all_list_items().await?;
    Ok(())
}
