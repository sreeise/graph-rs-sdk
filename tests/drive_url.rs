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
        .drives()
        .drive()
        .drive()
        .select(&["name"])
        .top("3");
    client.format_ord();
    client.url_mut(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/drives/drive?select=name&top=3",
            url.as_str()
        );
    });

    let _ = client
        .v1()
        .drives()
        .drive()
        .root()
        .expand(&["children"])
        .by_id("id");
    client.format_ord();
    client.url_mut(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/drives/id/root?expand=children",
            url.as_str()
        );
    });
}

#[test]
fn macro_endpoint_me_v1() {
    let client = get_drive();
    let _ = client.v1().me().drive().root();
    assert_url_special(&client, SpecialFolder::DriveRoot);
    let _ = client.v1().me().drive().root_children();
    assert_url_special(&client, SpecialFolder::DriveRootChild);
    let _ = client.v1().me().drive().recent();
    assert_url_special(&client, SpecialFolder::DriveRecent);
    let _ = client.v1().me().drive().delta();
    assert_url_special(&client, SpecialFolder::Delta);
    let _ = client.v1().me().drive().shared_with_me();
    assert_url_special(&client, SpecialFolder::SharedWithMe);
    let _ = client.v1().me().drive().special_documents();
    assert_url_special(&client, SpecialFolder::SpecialDocuments);
    let _ = client.v1().me().drive().special_documents_child();
    assert_url_special(&client, SpecialFolder::SpecialDocumentsChild);
    let _ = client.v1().me().drive().special_photos();
    assert_url_special(&client, SpecialFolder::SpecialPhotos);
    let _ = client.v1().me().drive().special_photos_child();
    assert_url_special(&client, SpecialFolder::SpecialPhotosChild);
    let _ = client.v1().me().drive().special_camera_roll();
    assert_url_special(&client, SpecialFolder::SpecialCameraRoll);
    let _ = client.v1().me().drive().special_camera_roll_child();
    assert_url_special(&client, SpecialFolder::SpecialCameraRollChild);
    let _ = client.v1().me().drive().special_app_root();
    assert_url_special(&client, SpecialFolder::SpecialAppRoot);
    let _ = client.v1().me().drive().special_app_root_child();
    assert_url_special(&client, SpecialFolder::SpecialAppRootChild);
    let _ = client.v1().me().drive().special_music();
    assert_url_special(&client, SpecialFolder::SpecialMusic);
    let _ = client.v1().me().drive().special_music_child();
    assert_url_special(&client, SpecialFolder::SpecialMusicChild);
}

#[test]
fn macro_endpoint_v1() {
    let client = get_drive();
    let _ = client.v1().drives().drive().root().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::DriveRoot);
    let _ = client.v1().drives().drive().root_children().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::DriveRootChild);
    let _ = client.v1().drives().drive().recent().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::DriveRecent);
    let _ = client.v1().drives().drive().delta().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::Delta);
    let _ = client.v1().drives().drive().shared_with_me().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SharedWithMe);
    let _ = client.v1().drives().drive().special_documents().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialDocuments);
    let _ = client
        .v1()
        .drives()
        .drive()
        .special_documents_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialDocumentsChild);
    let _ = client.v1().drives().drive().special_photos().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialPhotos);
    let _ = client
        .v1()
        .drives()
        .drive()
        .special_photos_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialPhotosChild);
    let _ = client.v1().drives().drive().special_camera_roll().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialCameraRoll);
    let _ = client
        .v1()
        .drives()
        .drive()
        .special_camera_roll_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialCameraRollChild);
    let _ = client.v1().drives().drive().special_app_root().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialAppRoot);
    let _ = client
        .v1()
        .drives()
        .drive()
        .special_app_root_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialAppRootChild);
    let _ = client.v1().drives().drive().special_music().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialMusic);
    let _ = client.v1().drives().drive().special_music_child().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialMusicChild);
}

static ID: &str = "01BYE5RZ2KXWOTNNU3K5B3AZ4YMANXEMAE";

#[test]
fn macro_endpoint_drives_v1_by_id() {
    let client = get_drive();
    let _ = client.v1().drives().drive().root().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::DriveRoot);
    let _ = client.v1().drives().drive().root_children().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::DriveRootChild);
    let _ = client.v1().drives().drive().recent().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::DriveRecent);
    let _ = client.v1().drives().drive().delta().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::Delta);
    let _ = client.v1().drives().drive().shared_with_me().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SharedWithMe);
    let _ = client.v1().drives().drive().special_documents().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialDocuments);
    let _ = client
        .v1()
        .drives()
        .drive()
        .special_documents_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialDocumentsChild);
    let _ = client.v1().drives().drive().special_photos().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialPhotos);
    let _ = client
        .v1()
        .drives()
        .drive()
        .special_photos_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialPhotosChild);
    let _ = client.v1().drives().drive().special_camera_roll().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialCameraRoll);
    let _ = client
        .v1()
        .drives()
        .drive()
        .special_camera_roll_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialCameraRollChild);
    let _ = client.v1().drives().drive().special_app_root().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialAppRoot);
    let _ = client
        .v1()
        .drives()
        .drive()
        .special_app_root_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialAppRootChild);
    let _ = client.v1().drives().drive().special_music().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialMusic);
    let _ = client.v1().drives().drive().special_music_child().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialMusicChild);
}

#[test]
fn macro_endpoint_sites_v1_by_id() {
    let client = get_drive();
    let _ = client.v1().sites().drive().root().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::DriveRoot);
    let _ = client.v1().sites().drive().root_children().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::DriveRootChild);
    let _ = client.v1().sites().drive().recent().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::DriveRecent);
    let _ = client.v1().sites().drive().delta().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::Delta);
    let _ = client.v1().sites().drive().shared_with_me().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SharedWithMe);
    let _ = client.v1().sites().drive().special_documents().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialDocuments);
    let _ = client
        .v1()
        .sites()
        .drive()
        .special_documents_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialDocumentsChild);
    let _ = client.v1().sites().drive().special_photos().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialPhotos);
    let _ = client.v1().sites().drive().special_photos_child().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialPhotosChild);
    let _ = client.v1().sites().drive().special_camera_roll().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialCameraRoll);
    let _ = client
        .v1()
        .sites()
        .drive()
        .special_camera_roll_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialCameraRollChild);
    let _ = client.v1().sites().drive().special_app_root().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialAppRoot);
    let _ = client
        .v1()
        .sites()
        .drive()
        .special_app_root_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialAppRootChild);
    let _ = client.v1().sites().drive().special_music().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialMusic);
    let _ = client.v1().sites().drive().special_music_child().by_id(ID);
    assert_url_id_equals(&client, ID, SpecialFolder::SpecialMusicChild);
}

#[test]
fn macro_endpoint_sites_beta_by_id() {
    let client = get_drive();
    let _ = client.beta().sites().drive().root().by_id(ID);
    assert_url_id_equals_beta(&client, ID, SpecialFolder::DriveRoot);
    let _ = client.beta().sites().drive().root_children().by_id(ID);
    assert_url_id_equals_beta(&client, ID, SpecialFolder::DriveRootChild);
    let _ = client.beta().sites().drive().recent().by_id(ID);
    assert_url_id_equals_beta(&client, ID, SpecialFolder::DriveRecent);
    let _ = client.beta().sites().drive().delta().by_id(ID);
    assert_url_id_equals_beta(&client, ID, SpecialFolder::Delta);
    let _ = client.beta().sites().drive().shared_with_me().by_id(ID);
    assert_url_id_equals_beta(&client, ID, SpecialFolder::SharedWithMe);
    let _ = client.beta().sites().drive().special_documents().by_id(ID);
    assert_url_id_equals_beta(&client, ID, SpecialFolder::SpecialDocuments);
    let _ = client
        .beta()
        .sites()
        .drive()
        .special_documents_child()
        .by_id(ID);
    assert_url_id_equals_beta(&client, ID, SpecialFolder::SpecialDocumentsChild);
    let _ = client.beta().sites().drive().special_photos().by_id(ID);
    assert_url_id_equals_beta(&client, ID, SpecialFolder::SpecialPhotos);
    let _ = client
        .beta()
        .sites()
        .drive()
        .special_photos_child()
        .by_id(ID);
    assert_url_id_equals_beta(&client, ID, SpecialFolder::SpecialPhotosChild);
    let _ = client
        .beta()
        .sites()
        .drive()
        .special_camera_roll()
        .by_id(ID);
    assert_url_id_equals_beta(&client, ID, SpecialFolder::SpecialCameraRoll);
    let _ = client
        .beta()
        .sites()
        .drive()
        .special_camera_roll_child()
        .by_id(ID);
    assert_url_id_equals_beta(&client, ID, SpecialFolder::SpecialCameraRollChild);
    let _ = client.beta().sites().drive().special_app_root().by_id(ID);
    assert_url_id_equals_beta(&client, ID, SpecialFolder::SpecialAppRoot);
    let _ = client
        .beta()
        .sites()
        .drive()
        .special_app_root_child()
        .by_id(ID);
    assert_url_id_equals_beta(&client, ID, SpecialFolder::SpecialAppRootChild);
    let _ = client.beta().sites().drive().special_music().by_id(ID);
    assert_url_id_equals_beta(&client, ID, SpecialFolder::SpecialMusic);
    let _ = client
        .beta()
        .sites()
        .drive()
        .special_music_child()
        .by_id(ID);
    assert_url_id_equals_beta(&client, ID, SpecialFolder::SpecialMusicChild);
}

#[test]
fn macro_endpoint_v2() {
    let client = get_drive();
    let _ = client.beta().me().drive().drive();
    assert_url_special_beta(&client, SpecialFolder::Drive);
    let _ = client.beta().me().drive().root();
    assert_url_special_beta(&client, SpecialFolder::DriveRoot);
    let _ = client.beta().me().drive().root_children();
    assert_url_special_beta(&client, SpecialFolder::DriveRootChild);
    let _ = client.beta().me().drive().recent();
    assert_url_special_beta(&client, SpecialFolder::DriveRecent);
    let _ = client.beta().me().drive().delta();
    assert_url_special_beta(&client, SpecialFolder::Delta);
    let _ = client.beta().me().drive().shared_with_me();
    assert_url_special_beta(&client, SpecialFolder::SharedWithMe);
    let _ = client.beta().me().drive().special_documents();
    assert_url_special_beta(&client, SpecialFolder::SpecialDocuments);
    let _ = client.beta().me().drive().special_documents_child();
    assert_url_special_beta(&client, SpecialFolder::SpecialDocumentsChild);
    let _ = client.beta().me().drive().special_photos();
    assert_url_special_beta(&client, SpecialFolder::SpecialPhotos);
    let _ = client.beta().me().drive().special_photos_child();
    assert_url_special_beta(&client, SpecialFolder::SpecialPhotosChild);
    let _ = client.beta().me().drive().special_camera_roll();
    assert_url_special_beta(&client, SpecialFolder::SpecialCameraRoll);
    let _ = client.beta().me().drive().special_camera_roll_child();
    assert_url_special_beta(&client, SpecialFolder::SpecialCameraRollChild);
    let _ = client.beta().me().drive().special_app_root();
    assert_url_special_beta(&client, SpecialFolder::SpecialAppRoot);
    let _ = client.beta().me().drive().special_app_root_child();
    assert_url_special_beta(&client, SpecialFolder::SpecialAppRootChild);
    let _ = client.beta().me().drive().special_music();
    assert_url_special_beta(&client, SpecialFolder::SpecialMusic);
    let _ = client.beta().me().drive().special_music_child();
    assert_url_special_beta(&client, SpecialFolder::SpecialMusicChild);
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
        .drives()
        .drive()
        .upload_session(
            "./test_files/item_test/complete_drive_item.json",
            Session::default(),
        )
        .by_path_id("32p99453", "Documents/complete_drive_item.json");
    assert_url_eq(
        &client,
        "/drives/32p99453/root:/Documents%2Fcomplete_drive_item.json:/createUploadSession",
    );
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
        .drives()
        .drive()
        .preview::<()>(None)
        .by_path_id("32p99453", "Documents/preview.txt");
    assert_url_eq(
        &client,
        "/drives/32p99453/root:/Documents%2Fpreview.txt:/preview",
    );

    let _ = client
        .v1()
        .sites()
        .drive()
        .preview::<()>(None)
        .by_path_id("32p99453", "Documents/preview.txt");
    assert_url_eq(
        &client,
        "/sites/32p99453/drive/root:/Documents%2Fpreview.txt:/preview",
    );

    let _ = client
        .v1()
        .groups()
        .drive()
        .preview::<()>(None)
        .by_path_id("32p99453", "Documents/preview.txt");
    assert_url_eq(
        &client,
        "/groups/32p99453/drive/root:/Documents%2Fpreview.txt:/preview",
    );

    let _ = client
        .v1()
        .users()
        .drive()
        .preview::<()>(None)
        .by_path_id("32p99453", "Documents/preview.txt");
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
        .drives()
        .drive()
        .create_folder("name", Some("replace"))
        .by_ids("132425", "4");
    assert_url_eq(&client, "/drives/4/items/132425/children");

    let _ = client
        .v1()
        .sites()
        .drive()
        .create_folder("name", Some("replace"))
        .by_ids("132425", "4");
    assert_url_eq(&client, "/sites/4/drive/items/132425/children");

    let _ = client
        .v1()
        .groups()
        .drive()
        .create_folder("name", Some("replace"))
        .by_ids("132425", "4");
    assert_url_eq(&client, "/groups/4/drive/items/132425/children");

    let _ = client
        .v1()
        .users()
        .drive()
        .create_folder("name", Some("replace"))
        .by_ids("132425", "4");
    assert_url_eq(&client, "/users/4/drive/items/132425/children");
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
        .drives()
        .drive()
        .create_folder("name", Some("replace"))
        .by_path_id("132534", "Documents");
    assert_url_eq(&client, "/drives/132534/root:/Documents:/children");

    let _ = client
        .v1()
        .sites()
        .drive()
        .create_folder("name", Some("replace"))
        .by_path_id("132534", "Documents");
    assert_url_eq(&client, "/sites/132534/drive/root:/Documents:/children");

    let _ = client
        .v1()
        .groups()
        .drive()
        .create_folder("name", Some("replace"))
        .by_path_id("132534", "Documents");
    assert_url_eq(&client, "/groups/132534/drive/root:/Documents:/children");

    let _ = client
        .v1()
        .users()
        .drive()
        .create_folder("name", Some("replace"))
        .by_path_id("132534", "Documents");
    assert_url_eq(&client, "/users/132534/drive/root:/Documents:/children");
}

#[test]
fn drive_delete() {
    let client = get_drive();
    let _ = client.v1().me().drive().delete().by_id("32p99453");
    assert_url_eq(&client, "/me/drive/items/32p99453");

    let _ = client
        .v1()
        .users()
        .drive()
        .delete()
        .by_ids("32p99453", "132534");
    assert_url_eq(&client, "/users/132534/drive/items/32p99453");
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
        .drives()
        .drive()
        .delete()
        .by_path_id("132534", "Documents/item.txt");
    assert_url_eq(&client, "/drives/132534/root:/Documents%2Fitem.txt:");
}

#[test]
fn drive_get_item() {
    let client = get_drive();
    let _ = client.v1().me().drive().get_item().by_id("32p99453");
    assert_url_eq(&client, "/me/drive/items/32p99453");

    let _ = client
        .v1()
        .drives()
        .drive()
        .get_item()
        .by_ids("132534", "32p99453");
    assert_url_eq(&client, "/drives/32p99453/items/132534");

    let _ = client
        .v1()
        .sites()
        .drive()
        .get_item()
        .by_ids("132534", "32p99453");
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
        .drives()
        .drive()
        .get_item()
        .by_path_id("32p99453", "Documents/item.txt");
    assert_url_eq(&client, "/drives/32p99453/root:/Documents%2Fitem.txt:");

    let _ = client
        .v1()
        .sites()
        .drive()
        .get_item()
        .by_path_id("32p99453", "Documents/item.txt");
    assert_url_eq(&client, "/sites/32p99453/drive/root:/Documents%2Fitem.txt:");
}

#[test]
fn drive_thumbnails() {
    let client = get_drive();
    let _ = client.v1().me().drive().thumbnails().by_id("32p99453");

    assert_url_eq(&client, "/me/drive/items/32p99453/thumbnails");

    let _ = client
        .v1()
        .drives()
        .drive()
        .thumbnails()
        .by_ids("132534", "32p99453");

    assert_url_eq(&client, "/drives/32p99453/items/132534/thumbnails");
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
        .drives()
        .drive()
        .single_thumbnail("4", "100")
        .by_ids("132534", "32p99453");
    assert_url_eq(&client, "/drives/32p99453/items/132534/thumbnails/4/100");

    let _ = client
        .v1()
        .sites()
        .drive()
        .single_thumbnail("4", "100")
        .by_ids("132534", "32p99453");
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
        .drives()
        .drive()
        .thumbnail_binary("4", "100")
        .by_ids("132534", "32p99453");
    assert_url_eq(
        &client,
        "/drives/32p99453/items/132534/thumbnails/4/100/content",
    );

    let _ = client
        .v1()
        .groups()
        .drive()
        .thumbnail_binary("4", "100")
        .by_ids("132534", "32p99453");
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
        .drives()
        .drive()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_ids("132534", "32p99453");
    assert_url_eq(
        &client,
        "/drives/32p99453/items/132534/drive_info.json/content",
    );

    let _ = client
        .v1()
        .sites()
        .drive()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_ids("132534", "32p99453");
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
        .drives()
        .drive()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_path_id("32p99453", "Documents/drive_info.json");
    assert_url_eq(
        &client,
        "/drives/32p99453/root:/Documents%2Fdrive_info.json:/content",
    );

    let _ = client
        .v1()
        .sites()
        .drive()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_path_id("32p99453", "Documents/drive_info.json");
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
        .drives()
        .drive()
        .upload_replace("./test_files/item_test/drive_info.json")
        .by_ids("132534", "32p99453");
    assert_url_eq(&client, "/drives/32p99453/items/132534/content");

    let _ = client
        .v1()
        .sites()
        .drive()
        .upload_replace("./test_files/item_test/drive_info.json")
        .by_ids("132534", "32p99453");
    assert_url_eq(&client, "/sites/32p99453/drive/items/132534/content");
}

#[test]
pub fn drive_list_versions() {
    let client = get_drive();
    let _ = client.v1().me().drive().list_versions().by_id("132534");
    assert_url_eq(&client, "/me/drive/items/132534/versions");

    let _ = client
        .v1()
        .drives()
        .drive()
        .list_versions()
        .by_ids("132534", "32p99453");
    assert_url_eq(&client, "/drives/32p99453/items/132534/versions");

    let _ = client
        .v1()
        .sites()
        .drive()
        .list_versions()
        .by_ids("132534", "32p99453");
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
        .drives()
        .drive()
        .list_versions()
        .by_path_id("32p99453", "Documents/item.txt");
    assert_url_eq(
        &client,
        "/drives/32p99453/root:/Documents%2Fitem.txt:/versions",
    );

    let _ = client
        .v1()
        .sites()
        .drive()
        .list_versions()
        .by_path_id("32p99453", "Documents/item.txt");
    assert_url_eq(
        &client,
        "/sites/32p99453/drive/root:/Documents%2Fitem.txt:/versions",
    );
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
        .drives()
        .drive()
        .restore_version("34492566a")
        .by_ids("132534", "32p99453");
    assert_url_eq(
        &client,
        "/drives/32p99453/items/132534/versions/34492566a/restoreVersion",
    );

    let _ = client
        .v1()
        .sites()
        .drive()
        .restore_version("34492566a")
        .by_ids("132534", "32p99453");
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
        .drives()
        .drive()
        .restore_version("34492566a")
        .by_path_id("32p99453", "Documents/item.txt");
    assert_url_eq(
        &client,
        "/drives/32p99453/root:/Documents%2Fitem.txt:/versions/34492566a/restoreVersion",
    );

    let _ = client
        .v1()
        .sites()
        .drive()
        .restore_version("34492566a")
        .by_path_id("32p99453", "Documents/item.txt");
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
        .drives()
        .drive()
        .download("./test_files")
        .by_ids("1234", "32p99453");
    assert_url_eq(&client, "/drives/32p99453/items/1234/content");

    let _ = client
        .v1()
        .sites()
        .drive()
        .download("./test_files")
        .by_ids("1234", "32p99453");
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
        .drives()
        .drive()
        .download("./test_files")
        .by_path_id("32p99453", "file.docx");
    assert_url_eq(&client, "/drives/32p99453/root:/file.docx:/content");

    let _ = client
        .v1()
        .sites()
        .drive()
        .download("./test_files")
        .by_path_id("32p99453", "file.docx");
    assert_url_eq(&client, "/sites/32p99453/drive/root:/file.docx:/content");
}

#[test]
pub fn drive_check_out() {
    let client = get_drive();
    let _ = client.v1().me().drive().check_out().by_id("1234");
    assert_url_eq(&client, "/me/drive/items/1234/checkout");

    let _ = client
        .v1()
        .drives()
        .drive()
        .check_out()
        .by_ids("1234", "32p99453");
    assert_url_eq(&client, "/drives/32p99453/items/1234/checkout");

    let _ = client
        .v1()
        .sites()
        .drive()
        .check_out()
        .by_ids("1234", "32p99453");
    assert_url_eq(&client, "/sites/32p99453/drive/items/1234/checkout");
}

#[test]
pub fn drive_check_in() {
    let client = get_drive();
    let _ = client.v1().me().drive().check_in(None, None).by_id("1234");
    assert_url_eq(&client, "/me/drive/items/1234/checkin");

    let _ = client
        .v1()
        .drives()
        .drive()
        .check_in(None, None)
        .by_ids("1234", "32p99453");
    assert_url_eq(&client, "/drives/32p99453/items/1234/checkin");

    let _ = client
        .v1()
        .sites()
        .drive()
        .check_in(None, None)
        .by_ids("1234", "32p99453");
    assert_url_eq(&client, "/sites/32p99453/drive/items/1234/checkin");
}

#[test]
pub fn drive_activities() {
    let client = get_drive();
    let _ = client.v1().me().drive().drive_activity();
    assert_url_eq(&client, "/me/drive/activities");

    let _ = client
        .v1()
        .drives()
        .drive()
        .drive_activity()
        .by_id("32p99453");
    assert_url_eq(&client, "/drives/32p99453/activities");

    let _ = client
        .v1()
        .sites()
        .drive()
        .drive_activity()
        .by_id("32p99453");
    assert_url_eq(&client, "/sites/32p99453/drive/activities");
}

#[test]
pub fn drive_item_activities() {
    let client = get_drive();
    let _ = client.v1().me().drive().item_activity().by_id("1234");
    assert_url_eq(&client, "/me/drive/items/1234/activities");

    let _ = client
        .v1()
        .drives()
        .drive()
        .item_activity()
        .by_ids("1234", "32p99453");
    assert_url_eq(&client, "/drives/32p99453/items/1234/activities");

    let _ = client
        .v1()
        .sites()
        .drive()
        .item_activity()
        .by_ids("1234", "32p99453");
    assert_url_eq(&client, "/sites/32p99453/drive/items/1234/activities");
}
