use graph_rs::http::Session;
use graph_rs::prelude::*;
use std::ffi::OsString;
use test_tools::drive::*;

fn get_drive() -> Graph {
    Graph::new("")
}

#[test]
fn query_mutate() {
    let client = get_drive();
    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .drive()
        .select(&["name"])
        .top("3");
    client.format_ord();
    client.url_mut(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/drives/32p99453?select=name&top=3",
            url.as_str()
        );
    });

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .root()
        .expand(&["children"]);
    client.format_ord();
    client.url_mut(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/drives/32p99453/root?expand=children",
            url.as_str()
        );
    });
}

#[test]
pub fn drive_upload_session() {
    let mut path = OsString::new();
    path.push("Documents/complete_drive_item.json");

    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .upload_session(
            "./test_files/item_test/complete_drive_item.json",
            Session::default(),
        )
        .by_path("Documents/complete_drive_item.json");
    assert_url_eq(
        &client,
        "/me/drive/root:/Documents%2Fcomplete_drive_item.json:/createUploadSession",
    );

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .upload_session(
            "./test_files/item_test/complete_drive_item.json",
            Session::default(),
        )
        .by_path("Documents/complete_drive_item.json");
    assert_url_eq(
        &client,
        "/drives/32p99453/root:/Documents%2Fcomplete_drive_item.json:/createUploadSession",
    );
}

#[test]
pub fn drive_main() {
    let client = get_drive();
    let _ = client.v1().me().drive().drive();
    assert_url_eq(&client, "/me/drive");

    let _ = client.v1().drives("32p99453").drive().drive();
    assert_url_eq(&client, "/drives/32p99453");
}

#[test]
pub fn drive_root() {
    let client = get_drive();
    let _ = client.v1().me().drive().root();
    assert_url_eq(&client, "/me/drive/root");

    let _ = client.v1().drives("32p99453").drive().root();
    assert_url_eq(&client, "/drives/32p99453/root");

    let _ = client.v1().sites("32p99453").drive().root();
    assert_url_eq(&client, "/sites/32p99453/drive/root");
}

#[test]
pub fn drive_recent() {
    let client = get_drive();
    let _ = client.v1().me().drive().recent();
    assert_url_eq(&client, "/me/drive/recent");

    let _ = client.v1().drives("32p99453").drive().recent();
    assert_url_eq(&client, "/drives/32p99453/recent");

    let _ = client.v1().sites("32p99453").drive().recent();
    assert_url_eq(&client, "/sites/32p99453/drive/recent");
}

#[test]
pub fn drive_delta() {
    let client = get_drive();
    let _ = client.v1().me().drive().delta();
    assert_url_eq(&client, "/me/drive/root/delta");

    let _ = client.v1().drives("32p99453").drive().delta();
    assert_url_eq(&client, "/drives/32p99453/root/delta");

    let _ = client.v1().sites("32p99453").drive().delta();
    assert_url_eq(&client, "/sites/32p99453/drive/root/delta");
}

#[test]
pub fn drive_root_children() {
    let client = get_drive();
    let _ = client.v1().me().drive().root_children();
    assert_url_eq(&client, "/me/drive/root/children");

    let _ = client.v1().drives("32p99453").drive().root_children();
    assert_url_eq(&client, "/drives/32p99453/root/children");

    let _ = client.v1().sites("32p99453").drive().root_children();
    assert_url_eq(&client, "/sites/32p99453/drive/root/children");
}

#[test]
pub fn drive_shared_with_me() {
    let client = get_drive();
    let _ = client.v1().me().drive().shared_with_me();
    assert_url_eq(&client, "/me/drive/sharedWithMe");

    let _ = client.v1().drives("32p99453").drive().shared_with_me();
    assert_url_eq(&client, "/drives/32p99453/sharedWithMe");

    let _ = client.v1().sites("32p99453").drive().shared_with_me();
    assert_url_eq(&client, "/sites/32p99453/drive/sharedWithMe");
}

#[test]
pub fn special_documents() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_documents();
    assert_url_eq(&client, "/me/drive/special/documents");

    let _ = client.v1().drives("32p99453").drive().special_documents();
    assert_url_eq(&client, "/drives/32p99453/special/documents");

    let _ = client.v1().sites("32p99453").drive().special_documents();
    assert_url_eq(&client, "/sites/32p99453/drive/special/documents");
}

#[test]
pub fn special_documents_children() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_documents_children();
    assert_url_eq(&client, "/me/drive/special/documents/children");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .special_documents_children();
    assert_url_eq(&client, "/drives/32p99453/special/documents/children");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .special_documents_children();
    assert_url_eq(&client, "/sites/32p99453/drive/special/documents/children");
}

#[test]
pub fn special_photos() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_photos();
    assert_url_eq(&client, "/me/drive/special/photos");

    let _ = client.v1().drives("32p99453").drive().special_photos();
    assert_url_eq(&client, "/drives/32p99453/special/photos");

    let _ = client.v1().sites("32p99453").drive().special_photos();
    assert_url_eq(&client, "/sites/32p99453/drive/special/photos");
}

#[test]
pub fn special_photos_children() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_photos_children();
    assert_url_eq(&client, "/me/drive/special/photos/children");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .special_photos_children();
    assert_url_eq(&client, "/drives/32p99453/special/photos/children");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .special_photos_children();
    assert_url_eq(&client, "/sites/32p99453/drive/special/photos/children");
}

#[test]
pub fn special_camera_roll() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_camera_roll();
    assert_url_eq(&client, "/me/drive/special/cameraroll");

    let _ = client.v1().drives("32p99453").drive().special_camera_roll();
    assert_url_eq(&client, "/drives/32p99453/special/cameraroll");

    let _ = client.v1().sites("32p99453").drive().special_camera_roll();
    assert_url_eq(&client, "/sites/32p99453/drive/special/cameraroll");
}

#[test]
pub fn special_camera_roll_children() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_camera_roll_children();
    assert_url_eq(&client, "/me/drive/special/cameraroll/children");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .special_camera_roll_children();
    assert_url_eq(&client, "/drives/32p99453/special/cameraroll/children");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .special_camera_roll_children();
    assert_url_eq(&client, "/sites/32p99453/drive/special/cameraroll/children");
}

#[test]
pub fn special_app_root() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_app_root();
    assert_url_eq(&client, "/me/drive/special/approot");

    let _ = client.v1().drives("32p99453").drive().special_app_root();
    assert_url_eq(&client, "/drives/32p99453/special/approot");

    let _ = client.v1().sites("32p99453").drive().special_app_root();
    assert_url_eq(&client, "/sites/32p99453/drive/special/approot");
}

#[test]
pub fn special_app_root_children() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_app_root_children();
    assert_url_eq(&client, "/me/drive/special/approot/children");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .special_app_root_children();
    assert_url_eq(&client, "/drives/32p99453/special/approot/children");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .special_app_root_children();
    assert_url_eq(&client, "/sites/32p99453/drive/special/approot/children");
}

#[test]
pub fn special_music() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_music();
    assert_url_eq(&client, "/me/drive/special/music");

    let _ = client.v1().drives("32p99453").drive().special_music();
    assert_url_eq(&client, "/drives/32p99453/special/music");

    let _ = client.v1().sites("32p99453").drive().special_music();
    assert_url_eq(&client, "/sites/32p99453/drive/special/music");
}

#[test]
pub fn special_music_children() {
    let client = get_drive();
    let _ = client.v1().me().drive().special_music_children();
    assert_url_eq(&client, "/me/drive/special/music/children");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .special_music_children();
    assert_url_eq(&client, "/drives/32p99453/special/music/children");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .special_music_children();
    assert_url_eq(&client, "/sites/32p99453/drive/special/music/children");
}

#[test]
pub fn drive_preview_path() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .preview::<()>(None)
        .by_path("Documents/preview.txt");
    assert_url_eq(&client, "/me/drive/root:/Documents%2Fpreview.txt:/preview");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .preview::<()>(None)
        .by_path("Documents/preview.txt");
    assert_url_eq(
        &client,
        "/drives/32p99453/root:/Documents%2Fpreview.txt:/preview",
    );

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .preview::<()>(None)
        .by_path("Documents/preview.txt");
    assert_url_eq(
        &client,
        "/sites/32p99453/drive/root:/Documents%2Fpreview.txt:/preview",
    );

    let _ = client
        .v1()
        .groups("32p99453")
        .drive()
        .preview::<()>(None)
        .by_path("Documents/preview.txt");
    assert_url_eq(
        &client,
        "/groups/32p99453/drive/root:/Documents%2Fpreview.txt:/preview",
    );

    let _ = client
        .v1()
        .users("32p99453")
        .drive()
        .preview::<()>(None)
        .by_path("Documents/preview.txt");
    assert_url_eq(
        &client,
        "/users/32p99453/drive/root:/Documents%2Fpreview.txt:/preview",
    );
}

#[test]
fn drive_create_folder() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .create_folder("name", Some("replace"))
        .by_id("132425");
    assert_url_eq(&client, "/me/drive/items/132425/children");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .create_folder("name", Some("replace"))
        .by_id("132425");
    assert_url_eq(&client, "/drives/32p99453/items/132425/children");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .create_folder("name", Some("replace"))
        .by_id("132425");
    assert_url_eq(&client, "/sites/32p99453/drive/items/132425/children");

    let _ = client
        .v1()
        .groups("32p99453")
        .drive()
        .create_folder("name", Some("replace"))
        .by_id("132425");
    assert_url_eq(&client, "/groups/32p99453/drive/items/132425/children");

    let _ = client
        .v1()
        .users("32p99453")
        .drive()
        .create_folder("name", Some("replace"))
        .by_id("132425");
    assert_url_eq(&client, "/users/32p99453/drive/items/132425/children");
}

#[test]
fn drive_create_folder_path() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .create_folder("name", Some("replace"))
        .by_path("Documents");
    assert_url_eq(&client, "/me/drive/root:/Documents:/children");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .create_folder("name", Some("replace"))
        .by_path("Documents");
    assert_url_eq(&client, "/drives/32p99453/root:/Documents:/children");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .create_folder("name", Some("replace"))
        .by_path("Documents");
    assert_url_eq(&client, "/sites/32p99453/drive/root:/Documents:/children");

    let _ = client
        .v1()
        .groups("32p99453")
        .drive()
        .create_folder("name", Some("replace"))
        .by_path("Documents");
    assert_url_eq(&client, "/groups/32p99453/drive/root:/Documents:/children");

    let _ = client
        .v1()
        .users("32p99453")
        .drive()
        .create_folder("name", Some("replace"))
        .by_path("Documents");
    assert_url_eq(&client, "/users/32p99453/drive/root:/Documents:/children");
}

#[test]
fn drive_delete() {
    let client = get_drive();
    let _ = client.v1().me().drive().delete().by_id("32p99453");
    assert_url_eq(&client, "/me/drive/items/32p99453");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .delete()
        .by_id("132534");
    assert_url_eq(&client, "/drives/32p99453/items/132534");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .delete()
        .by_id("132534");
    assert_url_eq(&client, "/sites/32p99453/drive/items/132534");
}

#[test]
fn drive_delete_path() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .delete()
        .by_path("Documents/item.txt");
    assert_url_eq(&client, "/me/drive/root:/Documents%2Fitem.txt:");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .delete()
        .by_path("Documents/item.txt");
    assert_url_eq(&client, "/drives/32p99453/root:/Documents%2Fitem.txt:");
}

#[test]
fn drive_get_item() {
    let client = get_drive();
    let _ = client.v1().me().drive().get_item().by_id("32p99453");
    assert_url_eq(&client, "/me/drive/items/32p99453");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .get_item()
        .by_id("132534");
    assert_url_eq(&client, "/drives/32p99453/items/132534");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .get_item()
        .by_id("132534");
    assert_url_eq(&client, "/sites/32p99453/drive/items/132534");
}

#[test]
fn drive_get_item_path() {
    let client = get_drive();

    let _ = client
        .v1()
        .me()
        .drive()
        .get_item()
        .by_path("Documents/item.txt");
    assert_url_eq(&client, "/me/drive/root:/Documents%2Fitem.txt:");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .get_item()
        .by_path("Documents/item.txt");
    assert_url_eq(&client, "/drives/32p99453/root:/Documents%2Fitem.txt:");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .get_item()
        .by_path("Documents/item.txt");
    assert_url_eq(&client, "/sites/32p99453/drive/root:/Documents%2Fitem.txt:");
}

#[test]
fn drive_thumbnails() {
    let client = get_drive();
    let _ = client.v1().me().drive().thumbnails().by_id("32p99453");
    assert_url_eq(&client, "/me/drive/items/32p99453/thumbnails");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .thumbnails()
        .by_id("132534");
    assert_url_eq(&client, "/drives/32p99453/items/132534/thumbnails");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .thumbnails()
        .by_id("132534");
    assert_url_eq(&client, "/sites/32p99453/drive/items/132534/thumbnails");
}

#[test]
fn drive_single_thumbnail() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .single_thumbnail("4", "100")
        .by_id("132534");
    assert_url_eq(&client, "/me/drive/items/132534/thumbnails/4/100");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .single_thumbnail("4", "100")
        .by_id("132534");
    assert_url_eq(&client, "/drives/32p99453/items/132534/thumbnails/4/100");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .single_thumbnail("4", "100")
        .by_id("132534");
    assert_url_eq(
        &client,
        "/sites/32p99453/drive/items/132534/thumbnails/4/100",
    );
}

#[test]
fn drive_thumbnail_binary() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .thumbnail_binary("4", "100")
        .by_id("132534");
    assert_url_eq(&client, "/me/drive/items/132534/thumbnails/4/100/content");
    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .thumbnail_binary("4", "100")
        .by_id("132534");
    assert_url_eq(
        &client,
        "/drives/32p99453/items/132534/thumbnails/4/100/content",
    );

    let _ = client
        .v1()
        .groups("32p99453")
        .drive()
        .thumbnail_binary("4", "100")
        .by_id("132534");
    assert_url_eq(
        &client,
        "/groups/32p99453/drive/items/132534/thumbnails/4/100/content",
    );
}

#[test]
pub fn drive_upload_new() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_id("132534");
    assert_url_eq(&client, "/me/drive/items/132534/drive_info.json/content");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_id("132534");
    assert_url_eq(
        &client,
        "/drives/32p99453/items/132534/drive_info.json/content",
    );

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_id("132534");
    assert_url_eq(
        &client,
        "/sites/32p99453/drive/items/132534/drive_info.json/content",
    );
}

#[test]
pub fn drive_upload_new_path() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_path("Documents/drive_info.json");
    assert_url_eq(
        &client,
        "/me/drive/root:/Documents%2Fdrive_info.json:/content",
    );

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_path("Documents/drive_info.json");
    assert_url_eq(
        &client,
        "/drives/32p99453/root:/Documents%2Fdrive_info.json:/content",
    );

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_path("Documents/drive_info.json");
    assert_url_eq(
        &client,
        "/sites/32p99453/drive/root:/Documents%2Fdrive_info.json:/content",
    );
}

#[test]
pub fn drive_upload_replace() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .upload_replace("./test_files/item_test/drive_info.json")
        .by_id("32p99453");
    assert_url_eq(&client, "/me/drive/items/32p99453/content");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .upload_replace("./test_files/item_test/drive_info.json")
        .by_id("132534");
    assert_url_eq(&client, "/drives/32p99453/items/132534/content");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .upload_replace("./test_files/item_test/drive_info.json")
        .by_id("132534");
    assert_url_eq(&client, "/sites/32p99453/drive/items/132534/content");
}

#[test]
pub fn drive_list_versions() {
    let client = get_drive();
    let _ = client.v1().me().drive().list_versions().by_id("132534");
    assert_url_eq(&client, "/me/drive/items/132534/versions");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .list_versions()
        .by_id("132534");
    assert_url_eq(&client, "/drives/32p99453/items/132534/versions");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .list_versions()
        .by_id("132534");
    assert_url_eq(&client, "/sites/32p99453/drive/items/132534/versions");
}

#[test]
pub fn drive_list_versions_path() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .list_versions()
        .by_path("Documents/item.txt");
    assert_url_eq(&client, "/me/drive/root:/Documents%2Fitem.txt:/versions");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .list_versions()
        .by_path("Documents/item.txt");
    assert_url_eq(
        &client,
        "/drives/32p99453/root:/Documents%2Fitem.txt:/versions",
    );

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .list_versions()
        .by_path("Documents/item.txt");
    assert_url_eq(
        &client,
        "/sites/32p99453/drive/root:/Documents%2Fitem.txt:/versions",
    );
}

#[test]
pub fn drive_list_children() {
    let client = get_drive();
    let _ = client.v1().me().drive().list_children().by_id("132534");
    assert_url_eq(&client, "/me/drive/items/132534/children");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .list_children()
        .by_id("132534");
    assert_url_eq(&client, "/drives/32p99453/items/132534/children");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .list_children()
        .by_id("132534");
    assert_url_eq(&client, "/sites/32p99453/drive/items/132534/children");
}

#[test]
pub fn drive_restore_version() {
    let client = get_drive();

    let _ = client
        .v1()
        .me()
        .drive()
        .restore_version("34492566a")
        .by_id("132534");
    assert_url_eq(
        &client,
        "/me/drive/items/132534/versions/34492566a/restoreVersion",
    );

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .restore_version("34492566a")
        .by_id("132534");
    assert_url_eq(
        &client,
        "/drives/32p99453/items/132534/versions/34492566a/restoreVersion",
    );

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .restore_version("34492566a")
        .by_id("132534");
    assert_url_eq(
        &client,
        "/sites/32p99453/drive/items/132534/versions/34492566a/restoreVersion",
    );
}

#[test]
pub fn drive_restore_version_path() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .restore_version("34492566a")
        .by_path("Documents/item.txt");
    assert_url_eq(
        &client,
        "/me/drive/root:/Documents%2Fitem.txt:/versions/34492566a/restoreVersion",
    );

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .restore_version("34492566a")
        .by_path("Documents/item.txt");
    assert_url_eq(
        &client,
        "/drives/32p99453/root:/Documents%2Fitem.txt:/versions/34492566a/restoreVersion",
    );

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .restore_version("34492566a")
        .by_path("Documents/item.txt");
    assert_url_eq(
        &client,
        "/sites/32p99453/drive/root:/Documents%2Fitem.txt:/versions/34492566a/restoreVersion",
    );
}

#[test]
pub fn drive_download() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .download("./test_files")
        .by_id("1234");
    assert_url_eq(&client, "/me/drive/items/1234/content");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .download("./test_files")
        .by_id("1234");
    assert_url_eq(&client, "/drives/32p99453/items/1234/content");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .download("./test_files")
        .by_id("1234");
    assert_url_eq(&client, "/sites/32p99453/drive/items/1234/content");
}

#[test]
pub fn drive_download_path() {
    let client = get_drive();
    let _ = client
        .v1()
        .me()
        .drive()
        .download("./test_files")
        .by_path("file.docx");
    assert_url_eq(&client, "/me/drive/root:/file.docx:/content");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .download("./test_files")
        .by_path("file.docx");
    assert_url_eq(&client, "/drives/32p99453/root:/file.docx:/content");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .download("./test_files")
        .by_path("file.docx");
    assert_url_eq(&client, "/sites/32p99453/drive/root:/file.docx:/content");
}

#[test]
pub fn drive_check_out() {
    let client = get_drive();
    let _ = client.v1().me().drive().check_out().by_id("1234");
    assert_url_eq(&client, "/me/drive/items/1234/checkout");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .check_out()
        .by_id("1234");
    assert_url_eq(&client, "/drives/32p99453/items/1234/checkout");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .check_out()
        .by_id("1234");
    assert_url_eq(&client, "/sites/32p99453/drive/items/1234/checkout");
}

#[test]
pub fn drive_check_in() {
    let client = get_drive();
    let _ = client.v1().me().drive().check_in(None, None).by_id("1234");
    assert_url_eq(&client, "/me/drive/items/1234/checkin");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .check_in(None, None)
        .by_id("1234");
    assert_url_eq(&client, "/drives/32p99453/items/1234/checkin");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .check_in(None, None)
        .by_id("1234");
    assert_url_eq(&client, "/sites/32p99453/drive/items/1234/checkin");
}

#[test]
pub fn drive_activities() {
    let client = get_drive();
    let _ = client.v1().me().drive().drive_activity();
    assert_url_eq(&client, "/me/drive/activities");

    let _ = client.v1().drives("32p99453").drive().drive_activity();
    assert_url_eq(&client, "/drives/32p99453/activities");

    let _ = client.v1().sites("32p99453").drive().drive_activity();
    assert_url_eq(&client, "/sites/32p99453/drive/activities");
}

#[test]
pub fn drive_item_activities() {
    let client = get_drive();
    let _ = client.v1().me().drive().item_activity().by_id("1234");
    assert_url_eq(&client, "/me/drive/items/1234/activities");

    let _ = client
        .v1()
        .drives("32p99453")
        .drive()
        .item_activity()
        .by_id("1234");
    assert_url_eq(&client, "/drives/32p99453/items/1234/activities");

    let _ = client
        .v1()
        .sites("32p99453")
        .drive()
        .item_activity()
        .by_id("1234");
    assert_url_eq(&client, "/sites/32p99453/drive/items/1234/activities");
}
