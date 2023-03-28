use graph_rs_sdk::prelude::*;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
fn list_notebooks() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/onenote/notebooks".to_string(),
        client
            .me()
            .onenote()
            .notebooks()
            .list_notebooks()
            .url()
            .path()
    );

    assert_eq!(
        format!("/sites/{}/onenote/notebooks", RID),
        client
            .site(RID)
            .onenote()
            .notebooks()
            .list_notebooks()
            .url()
            .path()
    );
}

#[test]
fn notebooks_list_sections() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/onenote/notebooks/{}/sections", ID),
        client
            .me()
            .onenote()
            .notebook(ID)
            .sections()
            .list_sections()
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/sites/{}/onenote/notebooks/{}/sections", RID, ID),
        client
            .site(RID)
            .onenote()
            .notebook(ID)
            .sections()
            .list_sections()
            .url()
            .path()
    );
}

#[test]
fn get_notebook() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/onenote/notebooks/{}", ID),
        client
            .me()
            .onenote()
            .notebook(ID)
            .get_notebooks()
            .url()
            .path()
    );
}

#[test]
fn create_notebook() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/onenote/notebooks".to_string(),
        client
            .me()
            .onenote()
            .notebooks()
            .create_notebooks(&String::new())
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/sites/{}/onenote/notebooks", RID),
        client
            .site(RID)
            .onenote()
            .notebooks()
            .create_notebooks(&String::new())
            .url()
            .path()
    );
}

#[test]
fn notebook_create_section() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/onenote/notebooks/{}/sections", ID),
        client
            .me()
            .onenote()
            .notebook(ID)
            .sections()
            .create_sections(&String::new())
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/sites/{}/onenote/notebooks/{}/sections", RID, ID),
        client
            .site(RID)
            .onenote()
            .notebook(ID)
            .sections()
            .create_sections(&String::new())
            .url()
            .path()
    );
}

#[test]
fn sections_copy_to_note_book() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/onenote/sections/{}/copyToNotebook", ID),
        client
            .me()
            .onenote()
            .section(ID)
            .copy_to_notebook(&String::new())
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/sites/{}/onenote/sections/{}/copyToNotebook", RID, ID),
        client
            .site(RID)
            .onenote()
            .section(ID)
            .copy_to_notebook(&String::new())
            .url()
            .path()
    );
}

#[test]
fn sections_copy_to_section_group() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/onenote/sections/{}/copyToSectionGroup", ID),
        client
            .me()
            .onenote()
            .section(ID)
            .copy_to_section_group(&String::new())
            .url()
            .path()
    );
}

#[test]
fn list_pages() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/onenote/pages".to_string(),
        client.me().onenote().pages().list_pages().url().path()
    );
}

#[test]
fn get_page() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/onenote/pages/{}", ID),
        client.me().onenote().page(ID).get_pages().url().path()
    );
}
