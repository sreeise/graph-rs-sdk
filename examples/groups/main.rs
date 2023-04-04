#![allow(dead_code)]

use graph_rs_sdk::error::GraphResult;

mod create_update_groups;
mod get_groups;
mod group_lifecycle_policies;

#[tokio::main]
async fn main() -> GraphResult<()> {
    create_update_groups::create_group().await?;
    create_update_groups::update_group().await?;
    get_groups::get_groups().await?;
    get_groups::list_groups().await?;
    group_lifecycle_policies::list_group_lifecycle_policies().await?;
    group_lifecycle_policies::add_group().await?;
    group_lifecycle_policies::create_group_lifecycle_policies().await?;
    Ok(())
}
