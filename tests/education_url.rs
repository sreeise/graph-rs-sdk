use graph_rs::http::BlockingHttpClient;
use graph_rs::prelude::Graph;

static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

fn get_graph() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn education_schools() {
    let client = get_graph();

    client.v1().education().schools().list_schools();

    client.url_ref(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/education/schools",
            url.as_str()
        );
    });

    client
        .v1()
        .education()
        .schools()
        .create_school(&serde_json::json!({}));

    client.url_ref(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/education/schools",
            url.as_str()
        );
    });

    client.v1().education().schools().get_school(ID);

    client.url_ref(|url| {
        assert_eq!(
            &format!("https://graph.microsoft.com/v1.0/education/schools/{}", ID),
            url.as_str()
        );
    });

    client
        .v1()
        .education()
        .schools()
        .update_school(ID, &serde_json::json!({}));

    client.url_ref(|url| {
        assert_eq!(
            &format!("https://graph.microsoft.com/v1.0/education/schools/{}", ID),
            url.as_str()
        );
    });

    client.v1().education().schools().delete_school(ID);

    client.url_ref(|url| {
        assert_eq!(
            &format!("https://graph.microsoft.com/v1.0/education/schools/{}", ID),
            url.as_str()
        );
    });

    client
        .v1()
        .education()
        .schools()
        .create_user(ID, &serde_json::json!({}));

    client.url_ref(|url| {
        assert_eq!(
            &format!(
                "https://graph.microsoft.com/v1.0/education/schools/{}/users/$ref",
                ID
            ),
            url.as_str()
        );
    });
}

#[test]
fn education_classes() {
    let client = get_graph();

    client.v1().education().classes().list_classes();

    client.url_ref(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/education/classes",
            url.as_str()
        );
    });

    client
        .v1()
        .education()
        .classes()
        .create_class(&serde_json::json!({}));

    client.url_ref(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/education/classes",
            url.as_str()
        );
    });

    client.v1().education().classes().get_class(ID);

    client.url_ref(|url| {
        assert_eq!(
            &format!("https://graph.microsoft.com/v1.0/education/classes/{}", ID),
            url.as_str()
        );
    });

    client
        .v1()
        .education()
        .classes()
        .update_class(ID, &serde_json::json!({}));

    client.url_ref(|url| {
        assert_eq!(
            &format!("https://graph.microsoft.com/v1.0/education/classes/{}", ID),
            url.as_str()
        );
    });

    client.v1().education().classes().delete_class(ID);

    client.url_ref(|url| {
        assert_eq!(
            &format!("https://graph.microsoft.com/v1.0/education/classes/{}", ID),
            url.as_str()
        );
    });

    client
        .v1()
        .education()
        .classes()
        .add_teacher(ID, &serde_json::json!({}));

    client.url_ref(|url| {
        assert_eq!(
            &format!(
                "https://graph.microsoft.com/v1.0/education/classes/{}/teachers/$ref",
                ID
            ),
            url.as_str()
        );
    });

    client.v1().education().classes().remove_teacher(ID, "2");

    client.url_ref(|url| {
        assert_eq!(
            &format!(
                "https://graph.microsoft.com/v1.0/education/classes/{}/teachers/{}/$ref",
                ID, "2"
            ),
            url.as_str()
        );
    });

    client.v1().education().classes().list_members(ID);

    client.url_ref(|url| {
        assert_eq!(
            &format!(
                "https://graph.microsoft.com/v1.0/education/classes/{}/members",
                ID
            ),
            url.as_str()
        );
    });

    client
        .v1()
        .education()
        .classes()
        .add_member(ID, &serde_json::json!({}));

    client.url_ref(|url| {
        assert_eq!(
            &format!(
                "https://graph.microsoft.com/v1.0/education/classes/{}/members/$ref",
                ID
            ),
            url.as_str()
        );
    });

    client.v1().education().classes().remove_member(ID, "2");

    client.url_ref(|url| {
        assert_eq!(
            &format!(
                "https://graph.microsoft.com/v1.0/education/classes/{}/members/{}/$ref",
                ID, "2"
            ),
            url.as_str()
        );
    });
}
