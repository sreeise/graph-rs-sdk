#[macro_use]
extern crate serde;

use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// See examples/paging for more examples.

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub(crate) id: Option<String>,
    #[serde(rename = "userPrincipalName")]
    user_principal_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub value: Vec<User>,
}

#[tokio::main]
async fn main() {
    paging().await.unwrap();
}

async fn paging() -> GraphResult<()> {
    let client = GraphClient::new(ACCESS_TOKEN);

    let deque = client
        .users()
        .list_user()
        .select(&["id", "userPrincipalName"])
        .paging()
        .json::<Users>()
        .await?;

    let users: Vec<User> = deque
        .into_iter()
        .flat_map(|resp| resp.into_body())
        .flat_map(|resp| resp.value)
        .collect();

    println!("{:#?}", users);
    Ok(())
}
