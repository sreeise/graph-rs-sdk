use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "<ACCESS_TOKEN>";

static GROUP_ID: &str = "<GROUP_ID>";

pub async fn get_groups() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client.group(GROUP_ID).get_group().send().await?;

    println!("{response:#?}");

    Ok(())
}

pub async fn list_groups() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client.groups().list_group().send().await.unwrap();

    println!("{response:#?}");

    Ok(())
}
