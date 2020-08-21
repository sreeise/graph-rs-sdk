use graph_rs::http::BlockingHttpClient;
use graph_rs::prelude::Graph;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

fn get_graph() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn activities_url() {
    let client = get_graph();
    client.v1().me().activities().list_activities();

    client.url_ref(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/me/activities",
            url.as_str()
        );
    });

    client.v1().users(RID).activities().list_activities();

    client.url_ref(|url| {
        assert_eq!(
            &format!("https://graph.microsoft.com/v1.0/users/{}/activities", RID),
            url.as_str()
        );
    });

    client.v1().me().activities().get_activities(ID);

    client.url_ref(|url| {
        assert_eq!(
            &format!("https://graph.microsoft.com/v1.0/me/activities/{}", ID),
            url.as_str()
        );
    });

    client.v1().users(RID).activities().get_activities(ID);

    client.url_ref(|url| {
        assert_eq!(
            &format!(
                "https://graph.microsoft.com/v1.0/users/{}/activities/{}",
                RID, ID
            ),
            url.as_str()
        );
    });
}
