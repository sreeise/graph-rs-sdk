use graph_http::BlockingHttpClient;
use graph_rs_sdk::prelude::Graph;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

fn get_graph() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn agreement_acceptances_url() {
    let client = get_graph();
    client
        .v1()
        .me()
        .agreement_acceptances()
        .list_agreement_acceptance();

    client.url_ref(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/me/agreementAcceptances",
            url.as_str()
        );
    });

    client
        .v1()
        .user(RID)
        .agreement_acceptances()
        .list_agreement_acceptance();

    client.url_ref(|url| {
        assert_eq!(
            &format!(
                "https://graph.microsoft.com/v1.0/users/{}/agreementAcceptances",
                RID
            ),
            url.as_str()
        );
    });

    client
        .v1()
        .user(RID)
        .agreement_acceptance(ID)
        .get_agreement_acceptance();

    client.url_ref(|url| {
        assert_eq!(
            &format!(
                "https://graph.microsoft.com/v1.0/users/{}/agreementAcceptances/{}",
                RID, ID
            ),
            url.as_str()
        );
    });

    client
        .v1()
        .agreement_acceptance(RID)
        .update_agreement_acceptance(&serde_json::json!({}));

    client.url_ref(|url| {
        assert_eq!(
            &format!(
                "https://graph.microsoft.com/v1.0/agreementAcceptances/{}",
                RID
            ),
            url.as_str()
        );
    });
}
