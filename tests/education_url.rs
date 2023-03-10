use graph_rs_sdk::client::GraphV2;

static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

fn get_graph() -> GraphV2 {
    GraphV2::new("token")
}

#[test]
fn education_schools() {
    let client = get_graph();

    assert_eq!(
        "/v1.0/education/schools".to_string(),
        client.education().schools().list_schools().url().path()
    );
    assert_eq!(
        "/v1.0/education/schools".to_string(),
        client
            .education()
            .schools()
            .create_schools(&String::new())
            .url()
            .path()
    );
    assert_eq!(
        format!("/v1.0/education/schools/{}", ID),
        client.education().school(ID).get_schools().url().path()
    );
}

#[test]
fn education_classes() {
    let client = get_graph();

    assert_eq!(
        "/v1.0/education/classes".to_string(),
        client.education().classes().list_classes().url().path()
    );
    assert_eq!(
        "/v1.0/education/classes".to_string(),
        client
            .education()
            .classes()
            .create_classes(&String::new())
            .url()
            .path()
    );
    assert_eq!(
        format!("/v1.0/education/classes/{}", ID),
        client.education().class(ID).get_classes().url().path()
    );
    assert_eq!(
        format!("/v1.0/education/classes/{}", ID),
        client
            .education()
            .class(ID)
            .update_classes(&String::new())
            .url()
            .path()
    );
    assert_eq!(
        format!("/v1.0/education/classes/{}/members", ID),
        client.education().class(ID).list_members().url().path()
    );
    assert_eq!(
        format!("/v1.0/education/classes/{}", ID),
        client.education().class(ID).delete_classes().url().path()
    );
}

#[test]
fn education_users() {
    let client = get_graph();

    assert_eq!(
        "/v1.0/education/users".to_string(),
        client.education().users().list_users().url().path()
    );
    assert_eq!(
        "/v1.0/education/users".to_string(),
        client
            .education()
            .users()
            .create_users(&String::new())
            .url()
            .path()
    );
    assert_eq!(
        format!("/v1.0/education/users/{}/classes/{}", ID, ID),
        client.education().user(ID).get_classes(ID).url().path()
    );
    assert_eq!(
        format!("/v1.0/education/users/{}", ID),
        client
            .education()
            .class(ID)
            .update_classes(&String::new())
            .url()
            .path()
    );
    assert_eq!(
        format!("/v1.0/education/users/{}/schools", ID),
        client.education().user(ID).list_schools().url().path()
    );
    assert_eq!(
        format!("/v1.0/education/users/{}/classes", ID),
        client.education().user(ID).list_classes().url().path()
    );
    assert_eq!(
        format!("/v1.0/education/users/{}", ID),
        client.education().user(ID).delete_users().url().path()
    );
}

#[test]
fn education_assignments() {
    let client = get_graph();

    assert_eq!(
        "/v1.0/education/me/assignments".to_string(),
        client
            .education()
            .me()
            .assignments()
            .list_assignments()
            .url()
            .path()
    );
    assert_eq!(
        format!("/v1.0/education/users/{}/assignments", ID),
        client
            .education()
            .user(ID)
            .assignments()
            .list_assignments()
            .url()
            .path()
    );
    assert_eq!(
        format!("/v1.0/education/users/{}/assignments/{}/rubric", ID, ID),
        client
            .education()
            .user(ID)
            .assignment(ID)
            .get_rubric()
            .url()
            .path()
    );
    assert_eq!(
        format!("/v1.0/education/users/{}/assignments/{}/categories", ID, ID),
        client
            .education()
            .user(ID)
            .assignment(ID)
            .list_categories()
            .url()
            .path()
    );
    assert_eq!(
        format!("/v1.0/education/users/{}/assignments/{}", ID, ID),
        client
            .education()
            .user(ID)
            .assignment(ID)
            .get_assignments()
            .url()
            .path()
    );
}

#[test]
fn education_assignments_submissions() {
    let client = get_graph();

    assert_eq!(
        format!(
            "/v1.0/education/users/{}/assignments/{}/submissions/$count",
            ID, ID
        ),
        client
            .education()
            .user(ID)
            .assignment(ID)
            .submissions()
            .get_submissions_count()
            .url()
            .path()
    );
    assert_eq!(
        format!(
            "/v1.0/education/users/{}/assignments/{}/submissions/{}/outcomes",
            ID, ID, ID
        ),
        client
            .education()
            .user(ID)
            .assignment(ID)
            .submission(ID)
            .create_outcomes(&String::new())
            .url()
            .path()
    );
    assert_eq!(
        format!(
            "/v1.0/education/schools/{}/assignments/{}/submissions",
            ID, ID
        ),
        client
            .education()
            .school(ID)
            .assignment(ID)
            .submissions()
            .list_submissions()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/education/schools/{}/assignments/{}/submissions/$count",
            ID, ID
        ),
        client
            .education()
            .school(ID)
            .assignment(ID)
            .submissions()
            .get_submissions_count()
            .url()
            .path()
    );
    assert_eq!(
        format!(
            "/v1.0/education/classes/{}/assignments/{}/submissions/{}/outcomes",
            ID, ID, ID
        ),
        client
            .education()
            .class(ID)
            .assignment(ID)
            .submission(ID)
            .create_outcomes(&String::new())
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/education/classes/{}/assignments/{}/submissions/$count",
            ID, ID
        ),
        client
            .education()
            .class(ID)
            .assignment(ID)
            .submissions()
            .get_submissions_count()
            .url()
            .path()
    );
    assert_eq!(
        format!(
            "/v1.0/education/classes/{}/assignments/{}/submissions/{}/outcomes",
            ID, ID, ID
        ),
        client
            .education()
            .class(ID)
            .assignment(ID)
            .submission(ID)
            .create_outcomes(&String::new())
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/education/classes/{}/assignments/{}/submissions/{}/outcomes/{}",
            ID, ID, ID, ID
        ),
        client
            .education()
            .class(ID)
            .assignment(ID)
            .submission(ID)
            .delete_outcomes(ID)
            .url()
            .path()
    );
}
