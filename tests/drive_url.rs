use graph_http::{BlockingHttpClient, Session};
use graph_rs_sdk::prelude::*;
use graph_rs_sdk::GRAPH_URL;
use test_tools::assert_url_eq;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

fn id_path(name: &str, middle: &str, last: Option<&str>) -> String {
    if let Some(l) = last {
        format!("/{}/{}/{}/{}/{}", name, RID, middle, ID, l)
    } else {
        format!("/{}/{}/{}/{}", name, RID, ID, middle)
    }
}

fn get_drive() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn query_mutate() {
    let client = get_drive();
    let _ = client
        .v1()
        .drive(RID)
        .get_drive()
        .select(&["name"])
        .top("3");
    client.url_ref(|url| {
		assert_eq!("https://graph.microsoft.com/v1.0/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x?%24select=name&%24top=3", url.as_str());
	});

    let _ = client.v1().drive(RID).get_root().expand(&["children"]);
    client.url_ref(|url| {
		assert_eq!("https://graph.microsoft.com/v1.0/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root?%24expand=children", url.as_str());
	});
}

#[test]
pub fn drive_upload_session() {
    let client = get_drive();
    let _ = client.v1().me().drive().create_upload_session(
        ":/Documents/complete_drive_item.json:",
        "./test_files/item_test/complete_drive_item.json",
        &Session::default(),
    );
    assert_url_eq(
        &client,
        "/me/drive/root:/Documents/complete_drive_item.json:/createUploadSession",
    );

    let _ = client.v1().drive(RID).create_upload_session(
        ":/Documents/complete_drive_item.json:",
        "./test_files/item_test/complete_drive_item.json",
        &Session::default(),
    );
    assert_url_eq(&client, "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root:/Documents/complete_drive_item.json:/createUploadSession");
}

#[test]
pub fn drive_main() {
    let client = get_drive();
    let _ = client.v1().me().drive().get_drive();
    assert_url_eq(&client, "/me/drive");

    let _ = client.v1().drive(RID).get_drive();
    assert_url_eq(&client, "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x");
}

#[test]
pub fn drive_root() {
    let client = get_drive();
    let _ = client.v1().me().drive().get_root();
    assert_url_eq(&client, "/me/drive/root");

    let _ = client.v1().drive(RID).get_root();
    assert_url_eq(&client, "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root");

    let _ = client.v1().site(RID).drive().get_root();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root",
    );
}

#[test]
pub fn drive_recent() {
    let client = get_drive();
    let _ = client.v1().me().drive().recent();
    assert_url_eq(&client, "/me/drive/recent()");

    let _ = client.v1().drive(RID).recent();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/recent()",
    );

    let _ = client.v1().site(RID).drive().recent();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/recent()",
    );
}

#[test]
pub fn drive_delta() {
    let client = get_drive();
    let _ = client.v1().me().drive().delta();
    assert_url_eq(&client, "/me/drive/root/delta");

    let _ = client.v1().drive(RID).delta();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root/delta",
    );

    let _ = client.v1().site(RID).drive().delta();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root/delta",
    );
}

#[test]
pub fn drive_root_children() {
    let client = get_drive();
    let _ = client.v1().me().drive().list_root_children();
    assert_url_eq(&client, "/me/drive/root/children");

    let _ = client.v1().drive(RID).list_root_children();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root/children",
    );

    let _ = client.v1().site(RID).drive().list_root_children();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root/children",
    );
}

#[test]
pub fn drive_shared_with_me() {
    let client = get_drive();
    let _ = client.v1().me().drive().shared_with_me();
    assert_url_eq(&client, "/me/drive/sharedWithMe()");

    let _ = client.v1().drive(RID).shared_with_me();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/sharedWithMe()",
    );

    let _ = client.v1().site(RID).drive().shared_with_me();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/sharedWithMe()",
    );
}

#[test]
pub fn special_documents() {
    let client = get_drive();
    let _ = client.v1().me().drive().list_special();
    assert_url_eq(&client, "/me/drive/special");

    let _ = client.v1().drive(RID).list_special();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/special",
    );

    let _ = client.v1().site(RID).drive().list_special();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/special",
    );
}

#[test]
pub fn special_documents_children() {
    let client = get_drive();
    let _ = client.v1().me().drive().get_special("documents/children");
    assert_url_eq(&client, "/me/drive/special/documents/children");

    let _ = client.v1().drive(RID).get_special("documents/children");
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/special/documents/children",
    );

    let _ = client
        .v1()
        .site(RID)
        .drive()
        .get_special("documents/children");
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/special/documents/children",
    );
}

#[test]
pub fn special_photos() {
    let client = get_drive();
    let _ = client.v1().me().drive().get_special("photos");
    assert_url_eq(&client, "/me/drive/special/photos");

    let _ = client.v1().drive(RID).get_special("photos");
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/special/photos",
    );

    let _ = client.v1().site(RID).drive().get_special("photos");
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/special/photos",
    );
}

#[test]
pub fn drive_preview_path() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .preview::<&str, ()>(":/Documents/preview.txt:", None);
    assert_url_eq(&client, "/me/drive/root:/Documents/preview.txt:/preview");

    let _ = client
        .v1()
        .drive(RID)
        .preview::<&str, ()>(":/Documents/preview.txt:", None);
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root:/Documents/preview.txt:/preview",
    );

    let _ = client
        .v1()
        .site(RID)
        .drive()
        .preview::<&str, ()>(":/Documents/preview.txt:", None);
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents/preview.txt:/preview",
    );

    let _ = client
        .v1()
        .user(RID)
        .drive()
        .preview::<&str, ()>(":/Documents/preview.txt:", None);
    assert_url_eq(
        &client,
        "/users/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents/preview.txt:/preview",
    );
}

#[test]
fn drive_create_folder() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .create_folder(ID, &serde_json::json!({}));
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/children");

    let _ = client
        .v1()
        .drive(RID)
        .create_folder(ID, &serde_json::json!({}));
    assert_url_eq(&client, id_path("drives", "items", Some("children")));

    let _ = client
        .v1()
        .site(RID)
        .drive()
        .create_folder(ID, &serde_json::json!({}));
    assert_url_eq(&client, id_path("sites", "drive/items", Some("children")));
}

#[test]
fn drive_create_folder_path() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .create_folder(":/Documents:", &serde_json::json!({}));
    assert_url_eq(&client, "/me/drive/root:/Documents:/children");

    let _ = client
        .v1()
        .drive(RID)
        .create_folder(":/Documents:", &serde_json::json!({}));
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root:/Documents:/children",
    );

    let _ = client
        .v1()
        .site(RID)
        .drive()
        .create_folder(":/Documents:", &serde_json::json!({}));
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents:/children",
    );

    let _ = client
        .v1()
        .user(RID)
        .drive()
        .create_folder(":/Documents:", &serde_json::json!({}));
    assert_url_eq(
        &client,
        "/users/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents:/children",
    );
}

#[test]
fn drive_delete() {
    let client = get_drive();
    let _ = client.v1().me().drive().delete_items(ID);
    assert_url_eq(
        &client,
        "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI",
    );

    let _ = client.v1().drive(RID).delete_items(ID);
    assert_url_eq(&client, "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI");

    let _ = client.v1().site(RID).drive().delete_items(ID);
    assert_url_eq(
		&client,
		"/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI"
	);
}

#[test]
fn drive_delete_path() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .delete_items(":/Documents/item.txt:");
    assert_url_eq(&client, "/me/drive/root:/Documents/item.txt:");

    let _ = client
        .v1()
        .drive("32p99453")
        .delete_items(":/Documents/item.txt:");
    assert_url_eq(&client, "/drives/32p99453/root:/Documents/item.txt:");
}

#[test]
fn drive_get_item() {
    let client = get_drive();
    let _ = client.v1().me().drive().get_items(ID);
    assert_url_eq(
        &client,
        "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI",
    );

    let _ = client.v1().drive(RID).get_items(ID);
    assert_url_eq(&client, "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI");

    let _ = client.v1().site(RID).drive().get_items(ID);
    assert_url_eq(
		&client,
		"/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI"
	);
}

#[test]
fn drive_get_item_path() {
    let client = get_drive();

    let _ = client.v1().me().drive().get_items(":/Documents/item.txt:");
    assert_url_eq(&client, "/me/drive/root:/Documents/item.txt:");

    let _ = client.v1().drive(RID).get_items(":/Documents/item.txt:");
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root:/Documents/item.txt:",
    );

    let _ = client
        .v1()
        .site(RID)
        .drive()
        .get_items(":/Documents/item.txt:");
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents/item.txt:",
    );
}

#[test]
fn drive_thumbnails() {
    let client = get_drive();
    let _ = client.v1().me().drive().list_thumbnails();
    assert_url_eq(&client, "/me/drive/items/thumbnails");

    let _ = client.v1().drive(RID).list_thumbnails();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/thumbnails",
    );

    let _ = client.v1().site(RID).drive().list_thumbnails();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/thumbnails",
    );
}

#[test]
fn drive_single_thumbnail() {
    let client = get_drive();
    let _ = client.v1().me().drive().get_thumbnail(ID, "4", "100");
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/thumbnails/4/100");

    let _ = client.v1().drive(RID).get_thumbnail(ID, "4", "100");
    assert_url_eq(
		&client,
		"/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/thumbnails/4/100"
	);

    let _ = client.v1().site(RID).drive().get_thumbnail(ID, "4", "100");
    assert_url_eq(
		&client,
		"/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/thumbnails/4/100"
	);
}

#[test]
fn drive_thumbnail_binary() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .get_thumbnail_binary(ID, "4", "100");
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/thumbnails/4/100/content");
    let _ = client.v1().drive(RID).get_thumbnail_binary(ID, "4", "100");
    assert_url_eq(
        &client,
        id_path("drives", "items", Some("thumbnails/4/100/content")),
    );
}

#[test]
pub fn drive_upload_new() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .upload_new(ID, "./test_files/item_test/drive_info.json");
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI:/drive_info.json:/content");

    let _ = client
        .v1()
        .drive(RID)
        .upload_new(ID, "./test_files/item_test/drive_info.json");
    assert_url_eq(
		&client,
		"/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI:/drive_info.json:/content"
	);

    let _ = client
        .v1()
        .site(RID)
        .drive()
        .upload_new(ID, "./test_files/item_test/drive_info.json");
    assert_url_eq(
		&client,
		"/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI:/drive_info.json:/content"
	);
}

#[test]
pub fn drive_upload_replace() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .upload_replace(ID, "./test_files/item_test/drive_info.json");
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/content");

    let _ = client
        .v1()
        .drive(RID)
        .upload_replace(ID, "./test_files/item_test/drive_info.json");
    assert_url_eq(&client, id_path("drives", "items", Some("content")));

    let _ = client
        .v1()
        .site(RID)
        .drive()
        .upload_replace(ID, "./test_files/item_test/drive_info.json");
    assert_url_eq(&client, id_path("sites", "drive/items", Some("content")));
}

#[test]
pub fn drive_list_versions() {
    let client = get_drive();
    let _ = client.v1().me().drive().list_item_versions(ID);
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/versions");

    let _ = client.v1().drive(RID).list_item_versions(ID);
    assert_url_eq(&client, id_path("drives", "items", Some("versions")));

    let _ = client.v1().site(RID).drive().list_item_versions(ID);
    assert_url_eq(&client, id_path("sites", "drive/items", Some("versions")));
}

#[test]
pub fn drive_list_versions_path() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .list_item_versions(":/Documents/item.txt:");
    assert_url_eq(&client, "/me/drive/root:/Documents/item.txt:/versions");

    let _ = client
        .v1()
        .drive(RID)
        .list_item_versions(":/Documents/item.txt:");
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root:/Documents/item.txt:/versions",
    );

    let _ = client
        .v1()
        .site(RID)
        .drive()
        .list_item_versions(":/Documents/item.txt:");
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents/item.txt:/versions",
    );
}

#[test]
pub fn drive_list_children() {
    let client = get_drive();
    let _ = client.v1().me().drive().list_children(ID);
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/children");

    let _ = client.v1().drive(RID).list_children(ID);
    assert_url_eq(
		&client,
		"/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/children"
	);

    let _ = client.v1().site(RID).drive().list_children(ID);
    assert_url_eq(
		&client,
		"/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/children"
	);

    let _ = client
        .v1()
        .me()
        .drive()
        .list_children(":/Documents/item.docx:");
    assert_url_eq(&client, "/me/drive/root:/Documents/item.docx:/children");

    let _ = client
        .v1()
        .drive(RID)
        .list_children(":/Documents/item.docx:");
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root:/Documents/item.docx:/children",
    );

    let _ = client
        .v1()
        .site(RID)
        .drive()
        .list_children(":/Documents/item.docx:");
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents/item.docx:/children",
    );
}

#[test]
pub fn drive_restore_version() {
    let client = get_drive();

    let _ = client
        .v1()
        .me()
        .drive()
        .restore_item_versions(ID, "34492566a");
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/versions/34492566a/restoreVersion");

    let _ = client
        .v1()
        .drive(RID)
        .restore_item_versions(ID, "34492566a");
    assert_url_eq(
		&client,
		"/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/versions/34492566a/restoreVersion"
	);

    let _ = client
        .v1()
        .site(RID)
        .drive()
        .restore_item_versions(ID, "34492566a");
    assert_url_eq(
		&client,
		"/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/versions/34492566a/restoreVersion"
	);
}

#[test]
pub fn drive_restore_version_path() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .restore_item_versions(":/Documents/item.txt:", "34492566a");
    assert_url_eq(
        &client,
        "/me/drive/root:/Documents/item.txt:/versions/34492566a/restoreVersion",
    );

    let _ = client
        .v1()
        .drive(RID)
        .restore_item_versions(":/Documents/item.txt:", "34492566a");
    assert_url_eq(&client, "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root:/Documents/item.txt:/versions/34492566a/restoreVersion");

    let _ = client
        .v1()
        .site(RID)
        .drive()
        .restore_item_versions(":/Documents/item.txt:", "34492566a");
    assert_url_eq(&client, "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents/item.txt:/versions/34492566a/restoreVersion");
}

#[test]
pub fn drive_download() {
    let client = get_drive();
    let download_client = client.v1().me().drive().download(ID, "./test_files");
    assert_eq!(
		download_client.url().to_string(),
		format!("{}/{}", GRAPH_URL, "me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/content")
	);

    let download_client = client.v1().drive(RID).download(ID, "./test_files");
    assert_eq!(
		download_client.url().to_string(),
		format!(
			"{}/{}",
			GRAPH_URL,
			"drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/content"
		)
	);

    let download_client = client.v1().site(RID).drive().download(ID, "./test_files");
    assert_eq!(
		download_client.url().to_string(),
		format!(
			"{}/{}",
			GRAPH_URL,
			"sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/content"
		)
	);
}

#[test]
pub fn drive_download_path() {
    let client = get_drive();
    let download_client = client
        .v1()
        .me()
        .drive()
        .download(":/file.docx:", "./test_files");
    assert_eq!(
        download_client.url().to_string(),
        format!("{}/{}", GRAPH_URL, "me/drive/root:/file.docx:/content")
    );

    let download_client = client
        .v1()
        .drive(RID)
        .download(":/file.docx:", "./test_files");
    assert_eq!(
        download_client.url().to_string(),
        format!(
            "{}/{}",
            GRAPH_URL, "drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root:/file.docx:/content"
        )
    );

    let download_client = client
        .v1()
        .site(RID)
        .drive()
        .download(":/file.docx:", "./test_files");
    assert_eq!(
        download_client.url().to_string(),
        format!(
            "{}/{}",
            GRAPH_URL, "sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/file.docx:/content"
        )
    );
}

#[test]
pub fn drive_check_out() {
    let client = get_drive();
    let _ = client.v1().me().drive().check_out_item(ID);
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/checkout");

    let _ = client.v1().drive(RID).check_out_item(ID);
    assert_url_eq(
		&client,
		"/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/checkout"
	);

    let _ = client.v1().site(RID).drive().check_out_item(ID);
    assert_url_eq(
		&client,
		"/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/checkout"
	);
}

#[test]
pub fn drive_check_in() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .check_in_item(ID, &serde_json::json!({}));
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/checkin");

    let _ = client
        .v1()
        .drive(RID)
        .check_in_item(ID, &serde_json::json!({}));
    assert_url_eq(
		&client,
		"/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/checkin"
	);

    let _ = client
        .v1()
        .site(RID)
        .drive()
        .check_in_item(ID, &serde_json::json!({}));
    assert_url_eq(
		&client,
		"/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/checkin"
	);
}

#[test]
pub fn drive_activities() {
    let client = get_drive();
    let _ = client.v1().me().drive().list_root_activities();
    assert_url_eq(&client, "/me/drive/activities");

    let _ = client.v1().drive(RID).list_root_activities();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/activities",
    );

    let _ = client.v1().site(RID).drive().list_root_activities();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/activities",
    );
}

#[test]
pub fn drive_item_activities() {
    let client = get_drive();
    let _ = client.v1().me().drive().get_item_activities(ID);
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/activities");

    let _ = client.v1().drive(RID).get_item_activities(ID);
    assert_url_eq(
		&client,
		"/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/activities"
	);

    let _ = client.v1().site(RID).drive().get_item_activities(ID);
    assert_url_eq(
		&client,
		"/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/activities"
	);

    let _ = client
        .v1()
        .drive(RID)
        .get_item_activities(":/Documents/item.txt:");
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root:/Documents/item.txt:/activities",
    );

    let _ = client
        .v1()
        .site(RID)
        .drive()
        .get_item_activities(":/Documents/item.txt:");
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents/item.txt:/activities",
    );
}
