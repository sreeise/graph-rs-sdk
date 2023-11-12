use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

static OWNER_ID: &str = "OWNER_ID";

pub async fn create_team() {
    let json = serde_json::json!({
      "template@odata.bind": "https://graph.microsoft.com/v1.0/teamsTemplates('standard')",
      "displayName": "Team Graph",
      "description": "This is a test description",
      "members":[{
       "@odata.type":"#microsoft.graph.aadUserConversationMember",
       "roles":[
          "owner"
       ],
       "user@odata.bind": format!("https://graph.microsoft.com/v1.0/users('{OWNER_ID}')")
    }]});

    let client = GraphClient::new(ACCESS_TOKEN);
    let response = client.teams().create_team(&json).send().await.unwrap();

    println!("{response:#?}");
}
