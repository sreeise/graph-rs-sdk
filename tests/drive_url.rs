use graph_rs::http::Session;
use graph_rs::prelude::*;
use test_tools::drive::*;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

fn id_path(name: &str, middle: &str, last: Option<&str>) -> String {
    if let Some(l) = last {
        format!("/{}/{}/{}/{}/{}", name, RID, middle, ID, l)
    } else {
        format!("/{}/{}/{}/{}", name, RID, ID, middle)
    }
}

fn get_drive() -> Graph {
    Graph::new("")
}

#[test]
fn query_mutate() {
    let client = get_drive();
    let _ = client
        .v1()
        .drives(RID)
        .drive()
        .drive()
        .select(&["name"])
        .top("3");
    client.url_mut(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x?select=name&top=3",
            url.as_str()
        );
    });

    let _ = client.v1().drives(RID).drive().root().expand(&["children"]);
    client.url_mut(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root?expand=children",
            url.as_str()
        );
    });
}

#[test]
pub fn drive_upload_session() {
    let client = get_drive();
    let _ = client.v1().me().drive().upload_session(
        ":/Documents/complete_drive_item.json:",
        "./test_files/item_test/complete_drive_item.json",
        Session::default(),
    );
    assert_url_eq(
        &client,
        "/me/drive/root:/Documents/complete_drive_item.json:/createUploadSession",
    );

    let _ = client.v1().drives(RID).drive().upload_session(
        ":/Documents/complete_drive_item.json:",
        "./test_files/item_test/complete_drive_item.json",
        Session::default(),
    );
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root:/Documents/complete_drive_item.json:/createUploadSession",
    );
}

#[test]
pub fn drive_main() {
    let client = get_drive();
    let _ = client.v1().me().drive().drive();
    assert_url_eq(&client, "/me/drive");

    let _ = client.v1().drives(RID).drive().drive();
    assert_url_eq(&client, "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x");
}

#[test]
pub fn drive_root() {
    let client = get_drive();
    let _ = client.v1().me().drive().root();
    assert_url_eq(&client, "/me/drive/root");

    let _ = client.v1().drives(RID).drive().root();
    assert_url_eq(&client, "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root");

    let _ = client.v1().sites(RID).drive().root();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root",
    );
}

#[test]
pub fn drive_recent() {
    let client = get_drive();
    let _ = client.v1().me().drive().recent();
    assert_url_eq(&client, "/me/drive/recent");

    let _ = client.v1().drives(RID).drive().recent();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/recent",
    );

    let _ = client.v1().sites(RID).drive().recent();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/recent",
    );
}

#[test]
pub fn drive_delta() {
    let client = get_drive();
    let _ = client.v1().me().drive().delta();
    assert_url_eq(&client, "/me/drive/root/delta");

    let _ = client.v1().drives(RID).drive().delta();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root/delta",
    );

    let _ = client.v1().sites(RID).drive().delta();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root/delta",
    );
}

#[test]
pub fn drive_root_children() {
    let client = get_drive();
    let _ = client.v1().me().drive().root_children();
    assert_url_eq(&client, "/me/drive/root/children");

    let _ = client.v1().drives(RID).drive().root_children();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root/children",
    );

    let _ = client.v1().sites(RID).drive().root_children();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root/children",
    );
}

#[test]
pub fn drive_shared_with_me() {
    let client = get_drive();
    let _ = client.v1().me().drive().shared_with_me();
    assert_url_eq(&client, "/me/drive/sharedWithMe");

    let _ = client.v1().drives(RID).drive().shared_with_me();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/sharedWithMe",
    );

    let _ = client.v1().sites(RID).drive().shared_with_me();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/sharedWithMe",
    );
}

#[test]
pub fn special_documents() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_documents();
    assert_url_eq(&client, "/me/drive/special/documents");

    let _ = client.v1().drives(RID).drive().special_documents();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/special/documents",
    );

    let _ = client.v1().sites(RID).drive().special_documents();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/special/documents",
    );
}

#[test]
pub fn special_documents_children() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_documents_children();
    assert_url_eq(&client, "/me/drive/special/documents/children");

    let _ = client.v1().drives(RID).drive().special_documents_children();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/special/documents/children",
    );

    let _ = client.v1().sites(RID).drive().special_documents_children();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/special/documents/children",
    );
}

#[test]
pub fn special_photos() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_photos();
    assert_url_eq(&client, "/me/drive/special/photos");

    let _ = client.v1().drives(RID).drive().special_photos();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/special/photos",
    );

    let _ = client.v1().sites(RID).drive().special_photos();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/special/photos",
    );
}

#[test]
pub fn special_photos_children() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_photos_children();
    assert_url_eq(&client, "/me/drive/special/photos/children");

    let _ = client.v1().drives(RID).drive().special_photos_children();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/special/photos/children",
    );

    let _ = client.v1().sites(RID).drive().special_photos_children();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/special/photos/children",
    );
}

#[test]
pub fn special_camera_roll() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_camera_roll();
    assert_url_eq(&client, "/me/drive/special/cameraroll");

    let _ = client.v1().drives(RID).drive().special_camera_roll();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/special/cameraroll",
    );

    let _ = client.v1().sites(RID).drive().special_camera_roll();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/special/cameraroll",
    );
}

#[test]
pub fn special_camera_roll_children() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_camera_roll_children();
    assert_url_eq(&client, "/me/drive/special/cameraroll/children");

    let _ = client
        .v1()
        .drives(RID)
        .drive()
        .special_camera_roll_children();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/special/cameraroll/children",
    );

    let _ = client
        .v1()
        .sites(RID)
        .drive()
        .special_camera_roll_children();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/special/cameraroll/children",
    );
}

#[test]
pub fn special_app_root() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_app_root();
    assert_url_eq(&client, "/me/drive/special/approot");

    let _ = client.v1().drives(RID).drive().special_app_root();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/special/approot",
    );

    let _ = client.v1().sites(RID).drive().special_app_root();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/special/approot",
    );
}

#[test]
pub fn special_app_root_children() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_app_root_children();
    assert_url_eq(&client, "/me/drive/special/approot/children");

    let _ = client.v1().drives(RID).drive().special_app_root_children();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/special/approot/children",
    );

    let _ = client.v1().sites(RID).drive().special_app_root_children();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/special/approot/children",
    );
}

#[test]
pub fn special_music() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_music();
    assert_url_eq(&client, "/me/drive/special/music");

    let _ = client.v1().drives(RID).drive().special_music();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/special/music",
    );

    let _ = client.v1().sites(RID).drive().special_music();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/special/music",
    );
}

#[test]
pub fn special_music_children() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_music_children();
    assert_url_eq(&client, "/me/drive/special/music/children");

    let _ = client.v1().drives(RID).drive().special_music_children();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/special/music/children",
    );

    let _ = client.v1().sites(RID).drive().special_music_children();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/special/music/children",
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
        .drives(RID)
        .drive()
        .preview::<&str, ()>(":/Documents/preview.txt:", None);
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root:/Documents/preview.txt:/preview",
    );

    let _ = client
        .v1()
        .sites(RID)
        .drive()
        .preview::<&str, ()>(":/Documents/preview.txt:", None);
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents/preview.txt:/preview",
    );

    let _ = client
        .v1()
        .groups(RID)
        .drive()
        .preview::<&str, ()>(":/Documents/preview.txt:", None);
    assert_url_eq(
        &client,
        "/groups/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents/preview.txt:/preview",
    );

    let _ = client
        .v1()
        .users(RID)
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
        .drives(RID)
        .drive()
        .create_folder(ID, &serde_json::json!({}));
    assert_url_eq(&client, id_path("drives", "items", Some("children")));

    let _ = client
        .v1()
        .sites(RID)
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
        .drives(RID)
        .drive()
        .create_folder(":/Documents:", &serde_json::json!({}));
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root:/Documents:/children",
    );

    let _ = client
        .v1()
        .sites(RID)
        .drive()
        .create_folder(":/Documents:", &serde_json::json!({}));
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents:/children",
    );

    let _ = client
        .v1()
        .groups(RID)
        .drive()
        .create_folder(":/Documents:", &serde_json::json!({}));
    assert_url_eq(
        &client,
        "/groups/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents:/children",
    );

    let _ = client
        .v1()
        .users(RID)
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
    let _ = client.v1().me().drive().delete(ID);
    assert_url_eq(
        &client,
        "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI",
    );

    let _ = client.v1().drives(RID).drive().delete(ID);
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI",
    );

    let _ = client.v1().sites(RID).drive().delete(ID);
    assert_url_eq(&client, "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI");
}

#[test]
fn drive_delete_path() {
    let client = get_drive();
    let _ = client.v1().me().drive().delete(":/Documents/item.txt:");
    assert_url_eq(&client, "/me/drive/root:/Documents/item.txt:");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .delete(":/Documents/item.txt:");
    assert_url_eq(&client, "/drives/32p99453/root:/Documents/item.txt:");
}

#[test]
fn drive_get_item() {
    let client = get_drive();
    let _ = client.v1().me().drive().get_item(ID);
    assert_url_eq(
        &client,
        "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI",
    );

    let _ = client.v1().drives(RID).drive().get_item(ID);
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI",
    );

    let _ = client.v1().sites(RID).drive().get_item(ID);
    assert_url_eq(&client, "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI");
}

#[test]
fn drive_get_item_path() {
    let client = get_drive();

    let _ = client.v1().me().drive().get_item(":/Documents/item.txt:");
    assert_url_eq(&client, "/me/drive/root:/Documents/item.txt:");

    let _ = client
        .v1()
        .drives(RID)
        .drive()
        .get_item(":/Documents/item.txt:");
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root:/Documents/item.txt:",
    );

    let _ = client
        .v1()
        .sites(RID)
        .drive()
        .get_item(":/Documents/item.txt:");
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents/item.txt:",
    );
}

#[test]
fn drive_thumbnails() {
    let client = get_drive();
    let _ = client.v1().me().drive().thumbnails();
    assert_url_eq(&client, "/me/drive/items/thumbnails");

    let _ = client.v1().drives(RID).drive().thumbnails();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/thumbnails",
    );

    let _ = client.v1().sites(RID).drive().thumbnails();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/thumbnails",
    );
}

#[test]
fn drive_single_thumbnail() {
    let client = get_drive();
    let _ = client.v1().me().drive().single_thumbnail(ID, "4", "100");
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/thumbnails/4/100");

    let _ = client
        .v1()
        .drives(RID)
        .drive()
        .single_thumbnail(ID, "4", "100");
    assert_url_eq(&client, "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/thumbnails/4/100");

    let _ = client
        .v1()
        .sites(RID)
        .drive()
        .single_thumbnail(ID, "4", "100");
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/thumbnails/4/100",
    );
}

#[test]
fn drive_thumbnail_binary() {
    let client = get_drive();
    let _ = client.v1().me().drive().thumbnail_binary(ID, "4", "100");
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/thumbnails/4/100/content");
    let _ = client
        .v1()
        .drives(RID)
        .drive()
        .thumbnail_binary(ID, "4", "100");
    assert_url_eq(
        &client,
        id_path("drives", "items", Some("thumbnails/4/100/content")),
    );

    let _ = client
        .v1()
        .groups(RID)
        .drive()
        .thumbnail_binary(ID, "4", "100");
    assert_url_eq(
        &client,
        id_path("groups", "drive/items", Some("thumbnails/4/100/content")),
    );
}

#[test]
pub fn drive_upload_new() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .upload_new(ID, "./test_files/item_test/drive_info.json")
        .unwrap();
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/drive_info.json/content");

    let _ = client
        .v1()
        .drives(RID)
        .drive()
        .upload_new(ID, "./test_files/item_test/drive_info.json")
        .unwrap();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/drive_info.json/content",
    );

    let _ = client
        .v1()
        .sites(RID)
        .drive()
        .upload_new(ID, "./test_files/item_test/drive_info.json")
        .unwrap();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/drive_info.json/content",
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
    assert_url_eq(
        &client,
        "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/content",
    );

    let _ = client
        .v1()
        .drives(RID)
        .drive()
        .upload_replace(ID, "./test_files/item_test/drive_info.json");
    assert_url_eq(&client, id_path("drives", "items", Some("content")));

    let _ = client
        .v1()
        .sites(RID)
        .drive()
        .upload_replace(ID, "./test_files/item_test/drive_info.json");
    assert_url_eq(&client, id_path("sites", "drive/items", Some("content")));
}

#[test]
pub fn drive_list_versions() {
    let client = get_drive();
    let _ = client.v1().me().drive().list_versions(ID);
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/versions");

    let _ = client.v1().drives(RID).drive().list_versions(ID);
    assert_url_eq(&client, id_path("drives", "items", Some("versions")));

    let _ = client.v1().sites(RID).drive().list_versions(ID);
    assert_url_eq(&client, id_path("sites", "drive/items", Some("versions")));
}

#[test]
pub fn drive_list_versions_path() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .list_versions(":/Documents/item.txt:");
    assert_url_eq(&client, "/me/drive/root:/Documents/item.txt:/versions");

    let _ = client
        .v1()
        .drives(RID)
        .drive()
        .list_versions(":/Documents/item.txt:");
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root:/Documents/item.txt:/versions",
    );

    let _ = client
        .v1()
        .sites(RID)
        .drive()
        .list_versions(":/Documents/item.txt:");
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

    let _ = client.v1().drives(RID).drive().list_children(ID);
    assert_url_eq(&client, "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/children");

    let _ = client.v1().sites(RID).drive().list_children(ID);
    assert_url_eq(&client, "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/children");
}

#[test]
pub fn drive_restore_version() {
    let client = get_drive();

    let _ = client.v1().me().drive().restore_version(ID, "34492566a");
    assert_url_eq(
        &client,
        "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/versions/34492566a/restoreVersion",
    );

    let _ = client
        .v1()
        .drives(RID)
        .drive()
        .restore_version(ID, "34492566a");
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/versions/34492566a/restoreVersion",
    );

    let _ = client
        .v1()
        .sites(RID)
        .drive()
        .restore_version(ID, "34492566a");
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/versions/34492566a/restoreVersion",
    );
}

#[test]
pub fn drive_restore_version_path() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .restore_version(":/Documents/item.txt:", "34492566a");
    assert_url_eq(
        &client,
        "/me/drive/root:/Documents/item.txt:/versions/34492566a/restoreVersion",
    );

    let _ = client
        .v1()
        .drives(RID)
        .drive()
        .restore_version(":/Documents/item.txt:", "34492566a");
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root:/Documents/item.txt:/versions/34492566a/restoreVersion",
    );

    let _ = client
        .v1()
        .sites(RID)
        .drive()
        .restore_version(":/Documents/item.txt:", "34492566a");
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents/item.txt:/versions/34492566a/restoreVersion",
    );
}

#[test]
pub fn drive_download() {
    let client = get_drive();
    let _ = client.v1().me().drive().download(ID, "./test_files");
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/content");

    let _ = client.v1().drives(RID).drive().download(ID, "./test_files");
    assert_url_eq(&client, "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/content");

    let _ = client.v1().sites(RID).drive().download(ID, "./test_files");
    assert_url_eq(&client, "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/content");
}

#[test]
pub fn drive_download_path() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .download(":/file.docx:", "./test_files");
    assert_url_eq(&client, "/me/drive/root:/file.docx:/content");

    let _ = client
        .v1()
        .drives(RID)
        .drive()
        .download(":/file.docx:", "./test_files");
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root:/file.docx:/content",
    );

    let _ = client
        .v1()
        .sites(RID)
        .drive()
        .download(":/file.docx:", "./test_files");
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/file.docx:/content",
    );
}

#[test]
pub fn drive_check_out() {
    let client = get_drive();
    let _ = client.v1().me().drive().check_out(ID);
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/checkout");

    let _ = client.v1().drives(RID).drive().check_out(ID);
    assert_url_eq(&client, "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/checkout");

    let _ = client.v1().sites(RID).drive().check_out(ID);
    assert_url_eq(&client, "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/checkout");
}

#[test]
pub fn drive_check_in() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .check_in(ID, &serde_json::json!({}));
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/checkin");

    let _ = client
        .v1()
        .drives(RID)
        .drive()
        .check_in(ID, &serde_json::json!({}));
    assert_url_eq(&client, "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/checkin");

    let _ = client
        .v1()
        .sites(RID)
        .drive()
        .check_in(ID, &serde_json::json!({}));
    assert_url_eq(&client, "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/checkin");
}

#[test]
pub fn drive_activities() {
    let client = get_drive();
    let _ = client.v1().me().drive().drive_activity();
    assert_url_eq(&client, "/me/drive/activities");

    let _ = client.v1().drives(RID).drive().drive_activity();
    assert_url_eq(
        &client,
        "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/activities",
    );

    let _ = client.v1().sites(RID).drive().drive_activity();
    assert_url_eq(
        &client,
        "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/activities",
    );
}

#[test]
pub fn drive_item_activities() {
    let client = get_drive();
    let _ = client.v1().me().drive().item_activity(ID);
    assert_url_eq(&client, "/me/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/activities");

    let _ = client.v1().drives(RID).drive().item_activity(ID);
    assert_url_eq(&client, "/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/activities");

    let _ = client.v1().sites(RID).drive().item_activity(ID);
    assert_url_eq(&client, "/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/items/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/activities");
}
