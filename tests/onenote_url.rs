use graph_rs::prelude::*;
use test_tools::drive::assert_url_eq;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
fn list_notebooks() {
    let client = Graph::new("");
    client.v1().me().onenote().notebooks().list();
    assert_url_eq(&client, "/me/onenote/notebooks");

    client.v1().groups(RID).onenote().notebooks().list();
    assert_url_eq(&client, format!("/groups/{}/onenote/notebooks", RID));

    client.v1().sites(RID).onenote().notebooks().list();
    assert_url_eq(&client, format!("/sites/{}/onenote/notebooks", RID));
}

#[test]
fn notebooks_list_sections() {
    let client = Graph::new("");
    client.v1().me().onenote().notebooks().list_sections(ID);
    assert_url_eq(&client, format!("/me/onenote/notebooks/{}/sections", ID));

    client
        .v1()
        .sites(RID)
        .onenote()
        .notebooks()
        .list_sections(ID);
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/notebooks/{}/sections", RID, ID),
    );
}

#[test]
fn get_notebook() {
    let client = Graph::new("");
    client.v1().me().onenote().notebooks().get(ID);
    assert_url_eq(&client, format!("/me/onenote/notebooks/{}", ID));

    client.v1().sites(RID).onenote().notebooks().get(ID);
    assert_url_eq(&client, format!("/sites/{}/onenote/notebooks/{}", RID, ID));
}

#[test]
fn create_notebook() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .onenote()
        .notebooks()
        .create(&String::new());
    assert_url_eq(&client, "/me/onenote/notebooks");

    client
        .v1()
        .groups(RID)
        .onenote()
        .notebooks()
        .create(&String::new());
    assert_url_eq(&client, format!("/groups/{}/onenote/notebooks", RID));

    client
        .v1()
        .sites(RID)
        .onenote()
        .notebooks()
        .create(&String::new());
    assert_url_eq(&client, format!("/sites/{}/onenote/notebooks", RID));
}

#[test]
fn get_recent_notebooks() {
    let client = Graph::new("");
    client.v1().me().onenote().notebooks().recent(true);
    assert_url_eq(
        &client,
        "/me/onenote/notebooks/getRecentNotebooks(includePersonalNotebooks=true)",
    );

    client.v1().sites(RID).onenote().notebooks().recent(false);
    assert_url_eq(
        &client,
        format!(
            "/sites/{}/onenote/notebooks/getRecentNotebooks(includePersonalNotebooks=false)",
            RID
        ),
    );
}

#[test]
fn notebook_create_section() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .onenote()
        .notebooks()
        .create_section(ID, &String::new());
    assert_url_eq(&client, format!("/me/onenote/notebooks/{}/sections", ID));

    client
        .v1()
        .sites(RID)
        .onenote()
        .notebooks()
        .create_section(ID, &String::new());
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/notebooks/{}/sections", RID, ID),
    );
}

#[test]
fn copy_notebook() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .onenote()
        .notebooks()
        .copy(ID, &String::new());
    assert_url_eq(
        &client,
        format!("/me/onenote/notebooks/{}/copyNotebook", ID),
    );

    client
        .v1()
        .sites(RID)
        .onenote()
        .notebooks()
        .copy(ID, &String::new());
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/notebooks/{}/copyNotebook", RID, ID),
    );
}

#[test]
fn list_sections() {
    let client = Graph::new("");
    client.v1().me().onenote().sections().list();
    assert_url_eq(&client, "/me/onenote/sections");

    client.v1().sites(RID).onenote().sections().list();
    assert_url_eq(&client, format!("/sites/{}/onenote/sections", RID));
}

#[test]
fn list_section_pages() {
    let client = Graph::new("");
    client.v1().me().onenote().sections().list_pages(ID);
    assert_url_eq(&client, format!("/me/onenote/sections/{}/pages", ID));

    client.v1().sites(RID).onenote().sections().list_pages(ID);
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/sections/{}/pages", RID, ID),
    );
}

#[test]
fn get_section() {
    let client = Graph::new("");
    client.v1().me().onenote().sections().get(ID);
    assert_url_eq(&client, format!("/me/onenote/sections/{}", ID));

    client.v1().sites(RID).onenote().sections().get(ID);
    assert_url_eq(&client, format!("/sites/{}/onenote/sections/{}", RID, ID));
}

#[test]
fn sections_copy_to_note_book() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .onenote()
        .sections()
        .copy_to_notebook(ID, &String::new());
    assert_url_eq(
        &client,
        format!("/me/onenote/sections/{}/copyToNotebook", ID),
    );

    client
        .v1()
        .sites(RID)
        .onenote()
        .sections()
        .copy_to_notebook(ID, &String::new());
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/sections/{}/copyToNotebook", RID, ID),
    );
}

#[test]
fn sections_copy_to_section_group() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .onenote()
        .sections()
        .copy_to_section_group(ID, &String::new());
    assert_url_eq(
        &client,
        format!("/me/onenote/sections/{}/copyToSectionGroup", ID),
    );

    client
        .v1()
        .sites(RID)
        .onenote()
        .sections()
        .copy_to_section_group(ID, &String::new());
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/sections/{}/copyToSectionGroup", RID, ID),
    );
}

#[test]
fn list_section_group() {
    let client = Graph::new("");
    client.v1().me().onenote().section_group().list();
    assert_url_eq(&client, "/me/onenote/sectionGroups");

    client.v1().sites(RID).onenote().section_group().list();
    assert_url_eq(&client, format!("/sites/{}/onenote/sectionGroups", RID));
}

#[test]
fn section_group_list_sections() {
    let client = Graph::new("");
    client.v1().me().onenote().section_group().list_sections(ID);
    assert_url_eq(
        &client,
        format!("/me/onenote/sectionGroups/{}/sections", ID),
    );

    client
        .v1()
        .sites(RID)
        .onenote()
        .section_group()
        .list_sections(ID);
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/sectionGroups/{}/sections", RID, ID),
    );
}

#[test]
fn get_section_group() {
    let client = Graph::new("");
    client.v1().me().onenote().section_group().get(ID);
    assert_url_eq(&client, format!("/me/onenote/sectionGroups/{}", ID));

    client.v1().sites(RID).onenote().section_group().get(ID);
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/sectionGroups/{}", RID, ID),
    );
}

#[test]
fn create_section_group() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .onenote()
        .section_group()
        .create(ID, &String::new());
    assert_url_eq(
        &client,
        format!("/me/onenote/sectionGroups/{}/sectionGroups", ID),
    );

    client
        .v1()
        .sites(RID)
        .onenote()
        .section_group()
        .create(ID, &String::new());
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/sectionGroups/{}/sectionGroups", RID, ID),
    );
}

#[test]
fn section_group_create_section() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .onenote()
        .section_group()
        .create_section(ID, &String::new());
    assert_url_eq(
        &client,
        format!("/me/onenote/sectionGroups/{}/sections", ID),
    );

    client
        .v1()
        .sites(RID)
        .onenote()
        .section_group()
        .create_section(ID, &String::new());
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/sectionGroups/{}/sections", RID, ID),
    );
}

#[test]
fn list_pages() {
    let client = Graph::new("");
    client.v1().me().onenote().pages().list();
    assert_url_eq(&client, "/me/onenote/pages");

    client.v1().sites(RID).onenote().pages().list();
    assert_url_eq(&client, format!("/sites/{}/onenote/pages", RID));
}

#[test]
fn get_page() {
    let client = Graph::new("");
    client.v1().me().onenote().pages().get(ID);
    assert_url_eq(&client, format!("/me/onenote/pages/{}", ID));

    client.v1().sites(RID).onenote().pages().get(ID);
    assert_url_eq(&client, format!("/sites/{}/onenote/pages/{}", RID, ID));
}

#[test]
fn update_page() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .onenote()
        .pages()
        .update(ID, &String::new());
    assert_url_eq(&client, format!("/me/onenote/pages/{}/content", ID));

    client
        .v1()
        .sites(RID)
        .onenote()
        .pages()
        .update(ID, &String::new());
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/pages/{}/content", RID, ID),
    );
}

#[test]
fn delete_page() {
    let client = Graph::new("");
    client.v1().me().onenote().pages().delete(ID);
    assert_url_eq(&client, format!("/me/onenote/pages/{}", ID));

    client.v1().sites(RID).onenote().pages().delete(ID);
    assert_url_eq(&client, format!("/sites/{}/onenote/pages/{}", RID, ID));
}

#[test]
fn pages_copy_to_section() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .onenote()
        .pages()
        .copy_to_section(ID, &String::new());
    assert_url_eq(&client, format!("/me/onenote/pages/{}/copyToSection", ID));

    client
        .v1()
        .sites(RID)
        .onenote()
        .pages()
        .copy_to_section(ID, &String::new());
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/pages/{}/copyToSection", RID, ID),
    );
}
