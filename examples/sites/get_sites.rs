use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "<SITE_ID>";

static SITE_ID: &str = "<SITE_ID>";

pub async fn get_site() -> GraphResult<()> {
    let client = GraphClient::new(ACCESS_TOKEN);

    let response = client.site(SITE_ID).get_site().send().await?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json().await.unwrap();
    println!("{body:#?}");

    Ok(())
}

pub async fn list_sites() -> GraphResult<()> {
    let client = GraphClient::new(ACCESS_TOKEN);

    let response = client.site(SITE_ID).list_sites().send().await?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json().await.unwrap();
    println!("{body:#?}");

    Ok(())
}
