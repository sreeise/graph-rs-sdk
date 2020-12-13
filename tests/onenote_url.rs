use graph_rs_sdk::prelude::*;
use test_tools::assert_url_eq;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
fn list_notebooks() {
    let client = Graph::new("");
    client.v1().me().onenote().notebooks().list_notebooks();
    assert_url_eq(&client, "/me/onenote/notebooks");

    client.v1().site(RID).onenote().notebooks().list_notebooks();
    assert_url_eq(&client, format!("/sites/{}/onenote/notebooks", RID));
}

#[test]
fn notebooks_list_sections() {
    let client = Graph::new("");
    client.v1().me().onenote().notebook(ID).list_sections();
    assert_url_eq(&client, format!("/me/onenote/notebooks/{}/sections", ID));

    client.v1().site(RID).onenote().notebook(ID).list_sections();
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/notebooks/{}/sections", RID, ID),
    );
}

#[test]
fn get_notebook() {
    let client = Graph::new("");
    client.v1().me().onenote().notebook(ID).get_notebooks();
    assert_url_eq(&client, format!("/me/onenote/notebooks/{}", ID));

    client.v1().site(RID).onenote().notebook(ID).get_notebooks();
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
        .create_notebooks(&String::new());
    assert_url_eq(&client, "/me/onenote/notebooks");

    client
        .v1()
        .site(RID)
        .onenote()
        .notebooks()
        .create_notebooks(&String::new());
    assert_url_eq(&client, format!("/sites/{}/onenote/notebooks", RID));
}

#[test]
fn notebook_create_section() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .onenote()
        .notebook(ID)
        .create_sections(&String::new());
    assert_url_eq(&client, format!("/me/onenote/notebooks/{}/sections", ID));

    client
        .v1()
        .site(RID)
        .onenote()
        .notebook(ID)
        .create_sections(&String::new());
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
        .notebook(ID)
        .copy_notebook(&String::new());
    assert_url_eq(
        &client,
        format!("/me/onenote/notebooks/{}/copyNotebook", ID),
    );

    client
        .v1()
        .site(RID)
        .onenote()
        .notebook(ID)
        .copy_notebook(&String::new());
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/notebooks/{}/copyNotebook", RID, ID),
    );
}

#[test]
fn list_sections() {
    let client = Graph::new("");
    client.v1().me().onenote().sections().list_sections();
    assert_url_eq(&client, "/me/onenote/sections");

    client.v1().site(RID).onenote().sections().list_sections();
    assert_url_eq(&client, format!("/sites/{}/onenote/sections", RID));
}

#[test]
fn list_section_pages() {
    let client = Graph::new("");
    client.v1().me().onenote().section(ID).list_pages();
    assert_url_eq(&client, format!("/me/onenote/sections/{}/pages", ID));

    client.v1().site(RID).onenote().section(ID).list_pages();
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/sections/{}/pages", RID, ID),
    );
}

#[test]
fn get_section() {
    let client = Graph::new("");
    client.v1().me().onenote().section(ID).get_sections();
    assert_url_eq(&client, format!("/me/onenote/sections/{}", ID));

    client.v1().site(RID).onenote().section(ID).get_sections();
    assert_url_eq(&client, format!("/sites/{}/onenote/sections/{}", RID, ID));
}

#[test]
fn sections_copy_to_note_book() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .onenote()
        .section(ID)
        .copy_to_notebook(&String::new());
    assert_url_eq(
        &client,
        format!("/me/onenote/sections/{}/copyToNotebook", ID),
    );

    client
        .v1()
        .site(RID)
        .onenote()
        .section(ID)
        .copy_to_notebook(&String::new());
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
        .section(ID)
        .copy_to_section_group(&String::new());
    assert_url_eq(
        &client,
        format!("/me/onenote/sections/{}/copyToSectionGroup", ID),
    );

    client
        .v1()
        .site(RID)
        .onenote()
        .section(ID)
        .copy_to_section_group(&String::new());
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/sections/{}/copyToSectionGroup", RID, ID),
    );
}

#[test]
fn list_section_group() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .onenote()
        .section_groups()
        .list_section_groups();
    assert_url_eq(&client, "/me/onenote/sectionGroups");

    client
        .v1()
        .site(RID)
        .onenote()
        .section_groups()
        .list_section_groups();
    assert_url_eq(&client, format!("/sites/{}/onenote/sectionGroups", RID));
}

#[test]
fn section_group_list_sections() {
    let client = Graph::new("");
    client.v1().me().onenote().section_group(ID).list_sections();
    assert_url_eq(
        &client,
        format!("/me/onenote/sectionGroups/{}/sections", ID),
    );

    client
        .v1()
        .site(RID)
        .onenote()
        .section_group(ID)
        .list_sections();
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/sectionGroups/{}/sections", RID, ID),
    );
}

#[test]
fn get_section_group() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .onenote()
        .section_group(ID)
        .get_section_groups();
    assert_url_eq(&client, format!("/me/onenote/sectionGroups/{}", ID));

    client
        .v1()
        .site(RID)
        .onenote()
        .section_group(ID)
        .get_section_groups();
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
        .section_group(ID)
        .create_section_groups(&String::new());
    assert_url_eq(
        &client,
        format!("/me/onenote/sectionGroups/{}/sectionGroups", ID),
    );

    client
        .v1()
        .site(RID)
        .onenote()
        .section_group(ID)
        .create_section_groups(&String::new());
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
        .section_group(ID)
        .create_sections(&String::new());
    assert_url_eq(
        &client,
        format!("/me/onenote/sectionGroups/{}/sections", ID),
    );

    client
        .v1()
        .site(RID)
        .onenote()
        .section_group(ID)
        .create_sections(&String::new());
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/sectionGroups/{}/sections", RID, ID),
    );
}

#[test]
fn list_pages() {
    let client = Graph::new("");
    client.v1().me().onenote().pages().list_pages();
    assert_url_eq(&client, "/me/onenote/pages");

    client.v1().site(RID).onenote().pages().list_pages();
    assert_url_eq(&client, format!("/sites/{}/onenote/pages", RID));
}

#[test]
fn get_page() {
    let client = Graph::new("");
    client.v1().me().onenote().page(ID).get_pages();
    assert_url_eq(&client, format!("/me/onenote/pages/{}", ID));

    client.v1().site(RID).onenote().page(ID).get_pages();
    assert_url_eq(&client, format!("/sites/{}/onenote/pages/{}", RID, ID));
}

#[test]
fn update_page() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .onenote()
        .page(ID)
        .update_pages(&String::new());
    assert_url_eq(&client, format!("/me/onenote/pages/{}", ID));

    client
        .v1()
        .site(RID)
        .onenote()
        .page(ID)
        .update_pages(&String::new());
    assert_url_eq(&client, format!("/sites/{}/onenote/pages/{}", RID, ID));
}

#[test]
fn update_page_content() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .onenote()
        .page(ID)
        .update_page_content(&String::new());
    assert_url_eq(&client, format!("/me/onenote/pages/{}/content", ID));

    client
        .v1()
        .site(RID)
        .onenote()
        .page(ID)
        .update_page_content(&String::new());
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/pages/{}/content", RID, ID),
    );
}

#[test]
fn delete_page() {
    let client = Graph::new("");
    client.v1().me().onenote().page(ID).delete_pages();
    assert_url_eq(&client, format!("/me/onenote/pages/{}", ID));

    client.v1().site(RID).onenote().page(ID).delete_pages();
    assert_url_eq(&client, format!("/sites/{}/onenote/pages/{}", RID, ID));
}

#[test]
fn pages_copy_to_section() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .onenote()
        .page(ID)
        .copy_to_section(&String::new());
    assert_url_eq(&client, format!("/me/onenote/pages/{}/copyToSection", ID));

    client
        .v1()
        .site(RID)
        .onenote()
        .page(ID)
        .copy_to_section(&String::new());
    assert_url_eq(
        &client,
        format!("/sites/{}/onenote/pages/{}/copyToSection", RID, ID),
    );
}
