use graph_rs_sdk::client::Graph;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
fn activities_url() {
    let client = Graph::new("token");

    assert_eq!(
        "/v1.0/me/activities".to_string(),
        client.me().activities().list_activities().url().path()
    );

    assert_eq!(
        format!("/v1.0/users/{}/activities", RID),
        client.user(RID).activities().list_activities().url().path()
    );

    assert_eq!(
        format!("/v1.0/me/activities/{}", ID),
        client.me().activity(ID).get_activities().url().path()
    );

    assert_eq!(
        format!("/v1.0/users/{}/activities/{}", RID, ID),
        client.user(RID).activity(ID).get_activities().url().path()
    );
}
