use graph_http::BlockingHttpClient;
use graph_rs::prelude::Graph;

static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

fn get_graph() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn education_schools() {
    let client = get_graph();

    client.v1().education().list_schools();

    client.url_ref(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/education/schools",
            url.as_str()
        );
    });

    client.v1().education().create_schools(&String::new());

    client.url_ref(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/education/schools",
            url.as_str()
        );
    });

    client.v1().education().get_schools(ID);

    client.url_ref(|url| {
        assert_eq!(
            &format!("https://graph.microsoft.com/v1.0/education/schools/{}", ID),
            url.as_str()
        );
    });

    client.v1().education().update_schools(ID, &String::new());

    client.url_ref(|url| {
        assert_eq!(
            &format!("https://graph.microsoft.com/v1.0/education/schools/{}", ID),
            url.as_str()
        );
    });

    // TODO: Add back create delete school
    /*
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
           .create_users(ID, &serde_json::json!({}));

       client.url_ref(|url| {
           assert_eq!(
               &format!(
                   "https://graph.microsoft.com/v1.0/education/schools/{}/users/$ref",
                   ID
               ),
               url.as_str()
           );
       });
    */
}

#[test]
fn education_classes() {
    let client = get_graph();

    client.v1().education().list_classes();

    client.url_ref(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/education/classes",
            url.as_str()
        );
    });

    client
        .v1()
        .education()
        .create_classes(&serde_json::json!({}));

    client.url_ref(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/education/classes",
            url.as_str()
        );
    });

    client.v1().education().get_classes(ID);

    client.url_ref(|url| {
        assert_eq!(
            &format!("https://graph.microsoft.com/v1.0/education/classes/{}", ID),
            url.as_str()
        );
    });

    client
        .v1()
        .education()
        .update_classes(ID, &serde_json::json!({}));

    client.url_ref(|url| {
        assert_eq!(
            &format!("https://graph.microsoft.com/v1.0/education/classes/{}", ID),
            url.as_str()
        );
    });

    // TODO: Add back delete classes

    /*
        client.v1().education().classes().delete_class(ID);

    client.url_ref(|url| {
        assert_eq!(
            &format!("https://graph.microsoft.com/v1.0/education/classes/{}", ID),
            url.as_str()
        );
    });
     */

    // TODO: Add back create and teachers

    /*
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
     */

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

    // TODO: Add back create and delete member

    /*
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
     */
}
