use graph_rs_sdk::Graph;

static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
fn education_schools() {
    let client = Graph::new("");

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
        format!("/v1.0/education/schools/{ID}"),
        client.education().school(ID).get_schools().url().path()
    );
}

#[test]
fn education_classes() {
    let client = Graph::new("");

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
        format!("/v1.0/education/classes/{ID}"),
        client.education().class(ID).get_classes().url().path()
    );
    assert_eq!(
        format!("/v1.0/education/classes/{ID}"),
        client
            .education()
            .class(ID)
            .update_classes(&String::new())
            .url()
            .path()
    );
    assert_eq!(
        format!("/v1.0/education/classes/{ID}/members"),
        client.education().class(ID).list_members().url().path()
    );
    assert_eq!(
        format!("/v1.0/education/classes/{ID}"),
        client.education().class(ID).delete_classes().url().path()
    );
}

#[test]
fn education_users() {
    let client = Graph::new("");

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
        format!("/v1.0/education/users/{ID}/classes/{ID}"),
        client.education().user(ID).get_classes(ID).url().path()
    );
    assert_eq!(
        format!("/v1.0/education/users/{ID}/schools"),
        client.education().user(ID).list_schools().url().path()
    );
    assert_eq!(
        format!("/v1.0/education/users/{ID}/classes"),
        client.education().user(ID).list_classes().url().path()
    );
    assert_eq!(
        format!("/v1.0/education/users/{ID}"),
        client.education().user(ID).delete_users().url().path()
    );
}

#[test]
fn education_assignments() {
    let client = Graph::new("");

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
        format!("/v1.0/education/users/{ID}/assignments"),
        client
            .education()
            .user(ID)
            .assignments()
            .list_assignments()
            .url()
            .path()
    );
    assert_eq!(
        format!("/v1.0/education/users/{ID}/assignments/{ID}/rubric"),
        client
            .education()
            .user(ID)
            .assignment(ID)
            .get_rubric()
            .url()
            .path()
    );
    assert_eq!(
        format!("/v1.0/education/users/{ID}/assignments/{ID}/categories"),
        client
            .education()
            .user(ID)
            .assignment(ID)
            .list_categories()
            .url()
            .path()
    );
    assert_eq!(
        format!("/v1.0/education/users/{ID}/assignments/{ID}"),
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
fn users_assignments_submissions() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/education/users/{ID}/assignments/{ID}/submissions/$count"),
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
        format!("/v1.0/education/users/{ID}/assignments/{ID}/submissions/{ID}/outcomes"),
        client
            .education()
            .user(ID)
            .assignment(ID)
            .submission(ID)
            .create_outcomes(&String::new())
            .url()
            .path()
    );
}

#[test]
fn schools_assignment_submission() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/education/schools/{ID}/assignments/{ID}/submissions"),
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
        format!("/v1.0/education/schools/{ID}/assignments/{ID}/submissions/$count"),
        client
            .education()
            .school(ID)
            .assignment(ID)
            .submissions()
            .get_submissions_count()
            .url()
            .path()
    );
}

#[test]
fn classes_assignment_submission() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/education/classes/{ID}/assignments/{ID}/submissions/{ID}/outcomes"),
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
        format!("/v1.0/education/classes/{ID}/assignments/{ID}/submissions/$count"),
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
        format!("/v1.0/education/classes/{ID}/assignments/{ID}/submissions/{ID}/outcomes"),
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
        format!("/v1.0/education/classes/{ID}/assignments/{ID}/submissions/{ID}/outcomes/{ID}"),
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
