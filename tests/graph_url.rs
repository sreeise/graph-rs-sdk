use graph_rs::client::Ident;
use graph_rs::drive::{GRAPH_URL, GRAPH_URL_BETA};
use graph_rs::http::Session;
use graph_rs::prelude::*;
use std::ffi::OsString;

fn get_drive() -> Graph {
    Graph::new("")
}

#[test]
fn query_mutate() {
    let client = get_drive();
    let _ = client.v1().drives().drive().drive();

    client.select(&["name"]).top("3").format_ord();
    client.url_mut(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/drives/drive?select=name&top=3",
            url.as_str()
        );
    });

    let _ = client.v1().drives().drive().root().by_id("id");
    client.expand(&["children"]).format_ord();
    client.url_mut(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/drives/id/root?expand=children",
            url.as_str()
        );
    });
}

fn assert_url_equals(client: &Graph, endpoint: DriveEndPoint) {
    client.url_ref(|url| {
        if client.ident().eq(&Ident::Me) {
            assert_eq!(
                format!("{}/me/drive/{}", GRAPH_URL, endpoint.as_str()),
                url.to_string(),
            )
        } else if client.ident().eq(&Ident::Drives) {
            assert_eq!(
                format!("{}/{}", GRAPH_URL, endpoint.as_str()),
                url.to_string()
            )
        } else {
            assert_eq!(
                format!("{}/drive/{}", GRAPH_URL, endpoint.as_str()),
                url.to_string()
            )
        }
    })
}

fn assert_url_id_equals(client: &Graph, item_id: &str, endpoint: DriveEndPoint) {
    client.url_ref(|url| {
        if client.ident().eq(&Ident::Me) {
            assert_eq!(
                format!("{}/me/drive/{}", GRAPH_URL, endpoint.as_str()),
                url.to_string(),
            )
        } else if client.ident().eq(&Ident::Drives) {
            assert_eq!(
                format!(
                    "{}/{}/{}/{}",
                    GRAPH_URL,
                    client.ident().as_ref(),
                    item_id,
                    endpoint.as_str()
                ),
                url.to_string()
            )
        } else {
            assert_eq!(
                format!(
                    "{}/{}/{}/drive/{}",
                    GRAPH_URL,
                    client.ident().as_ref(),
                    item_id,
                    endpoint.as_str()
                ),
                url.to_string()
            )
        }
    })
}

fn assert_url_equals_beta(client: &Graph, endpoint: DriveEndPoint) {
    client.url_ref(|url| {
        if client.ident().eq(&Ident::Me) {
            if endpoint.eq(&DriveEndPoint::Drive) {
                assert_eq!(
                    format!("{}/me/{}", GRAPH_URL_BETA, endpoint.as_str()),
                    url.to_string(),
                )
            } else {
                assert_eq!(
                    format!("{}/me/drive/{}", GRAPH_URL_BETA, endpoint.as_str()),
                    url.to_string(),
                )
            }
        } else {
            if endpoint.eq(&DriveEndPoint::Drive) {
                assert_eq!(
                    format!("{}/{}", GRAPH_URL_BETA, endpoint.as_str()),
                    url.to_string()
                )
            } else {
                assert_eq!(
                    format!("{}/drive/{}", GRAPH_URL_BETA, endpoint.as_str()),
                    url.to_string()
                )
            }
        }
    })
}

fn assert_url_id_equals_beta(client: &Graph, item_id: &str, endpoint: DriveEndPoint) {
    client.url_ref(|url| {
        if client.ident().eq(&Ident::Me) {
            assert_eq!(
                format!("{}/me/drive/{}", GRAPH_URL_BETA, endpoint.as_str()),
                url.to_string(),
            )
        } else if client.ident().eq(&Ident::Drives) {
            assert_eq!(
                format!(
                    "{}/{}/{}/{}",
                    GRAPH_URL_BETA,
                    client.ident().as_ref(),
                    item_id,
                    endpoint.as_str()
                ),
                url.to_string()
            )
        } else {
            assert_eq!(
                format!(
                    "{}/{}/{}/drive/{}",
                    GRAPH_URL_BETA,
                    client.ident().as_ref(),
                    item_id,
                    endpoint.as_str()
                ),
                url.to_string()
            )
        }
    })
}

#[test]
fn macro_endpoint_me_v1() {
    let client = get_drive();
    let _ = client.v1().me().drive().root();
    assert_url_equals(&client, DriveEndPoint::DriveRoot);
    let _ = client.v1().me().drive().root_children();
    assert_url_equals(&client, DriveEndPoint::DriveRootChild);
    let _ = client.v1().me().drive().recent();
    assert_url_equals(&client, DriveEndPoint::DriveRecent);
    let _ = client.v1().me().drive().delta();
    assert_url_equals(&client, DriveEndPoint::Delta);
    let _ = client.v1().me().drive().shared_with_me();
    assert_url_equals(&client, DriveEndPoint::SharedWithMe);
    let _ = client.v1().me().drive().special_documents();
    assert_url_equals(&client, DriveEndPoint::SpecialDocuments);
    let _ = client.v1().me().drive().special_documents_child();
    assert_url_equals(&client, DriveEndPoint::SpecialDocumentsChild);
    let _ = client.v1().me().drive().special_photos();
    assert_url_equals(&client, DriveEndPoint::SpecialPhotos);
    let _ = client.v1().me().drive().special_photos_child();
    assert_url_equals(&client, DriveEndPoint::SpecialPhotosChild);
    let _ = client.v1().me().drive().special_camera_roll();
    assert_url_equals(&client, DriveEndPoint::SpecialCameraRoll);
    let _ = client.v1().me().drive().special_camera_roll_child();
    assert_url_equals(&client, DriveEndPoint::SpecialCameraRollChild);
    let _ = client.v1().me().drive().special_app_root();
    assert_url_equals(&client, DriveEndPoint::SpecialAppRoot);
    let _ = client.v1().me().drive().special_app_root_child();
    assert_url_equals(&client, DriveEndPoint::SpecialAppRootChild);
    let _ = client.v1().me().drive().special_music();
    assert_url_equals(&client, DriveEndPoint::SpecialMusic);
    let _ = client.v1().me().drive().special_music_child();
    assert_url_equals(&client, DriveEndPoint::SpecialMusicChild);
}

#[test]
fn macro_endpoint_v1() {
    let client = get_drive();
    let _ = client.v1().drives().drive().root().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::DriveRoot);
    let _ = client.v1().drives().drive().root_children().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::DriveRootChild);
    let _ = client.v1().drives().drive().recent().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::DriveRecent);
    let _ = client.v1().drives().drive().delta().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::Delta);
    let _ = client.v1().drives().drive().shared_with_me().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SharedWithMe);
    let _ = client.v1().drives().drive().special_documents().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialDocuments);
    let _ = client
        .v1()
        .drives()
        .drive()
        .special_documents_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialDocumentsChild);
    let _ = client.v1().drives().drive().special_photos().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialPhotos);
    let _ = client
        .v1()
        .drives()
        .drive()
        .special_photos_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialPhotosChild);
    let _ = client.v1().drives().drive().special_camera_roll().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialCameraRoll);
    let _ = client
        .v1()
        .drives()
        .drive()
        .special_camera_roll_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialCameraRollChild);
    let _ = client.v1().drives().drive().special_app_root().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialAppRoot);
    let _ = client
        .v1()
        .drives()
        .drive()
        .special_app_root_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialAppRootChild);
    let _ = client.v1().drives().drive().special_music().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialMusic);
    let _ = client.v1().drives().drive().special_music_child().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialMusicChild);
}

static ID: &str = "01BYE5RZ2KXWOTNNU3K5B3AZ4YMANXEMAE";

#[test]
fn macro_endpoint_drives_v1_by_id() {
    let client = get_drive();
    let _ = client.v1().drives().drive().root().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::DriveRoot);
    let _ = client.v1().drives().drive().root_children().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::DriveRootChild);
    let _ = client.v1().drives().drive().recent().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::DriveRecent);
    let _ = client.v1().drives().drive().delta().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::Delta);
    let _ = client.v1().drives().drive().shared_with_me().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SharedWithMe);
    let _ = client.v1().drives().drive().special_documents().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialDocuments);
    let _ = client
        .v1()
        .drives()
        .drive()
        .special_documents_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialDocumentsChild);
    let _ = client.v1().drives().drive().special_photos().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialPhotos);
    let _ = client
        .v1()
        .drives()
        .drive()
        .special_photos_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialPhotosChild);
    let _ = client.v1().drives().drive().special_camera_roll().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialCameraRoll);
    let _ = client
        .v1()
        .drives()
        .drive()
        .special_camera_roll_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialCameraRollChild);
    let _ = client.v1().drives().drive().special_app_root().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialAppRoot);
    let _ = client
        .v1()
        .drives()
        .drive()
        .special_app_root_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialAppRootChild);
    let _ = client.v1().drives().drive().special_music().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialMusic);
    let _ = client.v1().drives().drive().special_music_child().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialMusicChild);
}

#[test]
fn macro_endpoint_sites_v1_by_id() {
    let client = get_drive();
    let _ = client.v1().sites().drive().root().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::DriveRoot);
    let _ = client.v1().sites().drive().root_children().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::DriveRootChild);
    let _ = client.v1().sites().drive().recent().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::DriveRecent);
    let _ = client.v1().sites().drive().delta().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::Delta);
    let _ = client.v1().sites().drive().shared_with_me().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SharedWithMe);
    let _ = client.v1().sites().drive().special_documents().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialDocuments);
    let _ = client
        .v1()
        .sites()
        .drive()
        .special_documents_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialDocumentsChild);
    let _ = client.v1().sites().drive().special_photos().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialPhotos);
    let _ = client.v1().sites().drive().special_photos_child().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialPhotosChild);
    let _ = client.v1().sites().drive().special_camera_roll().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialCameraRoll);
    let _ = client
        .v1()
        .sites()
        .drive()
        .special_camera_roll_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialCameraRollChild);
    let _ = client.v1().sites().drive().special_app_root().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialAppRoot);
    let _ = client
        .v1()
        .sites()
        .drive()
        .special_app_root_child()
        .by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialAppRootChild);
    let _ = client.v1().sites().drive().special_music().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialMusic);
    let _ = client.v1().sites().drive().special_music_child().by_id(ID);
    assert_url_id_equals(&client, ID, DriveEndPoint::SpecialMusicChild);
}

#[test]
fn macro_endpoint_sites_beta_by_id() {
    let client = get_drive();
    let _ = client.beta().sites().drive().root().by_id(ID);
    assert_url_id_equals_beta(&client, ID, DriveEndPoint::DriveRoot);
    let _ = client.beta().sites().drive().root_children().by_id(ID);
    assert_url_id_equals_beta(&client, ID, DriveEndPoint::DriveRootChild);
    let _ = client.beta().sites().drive().recent().by_id(ID);
    assert_url_id_equals_beta(&client, ID, DriveEndPoint::DriveRecent);
    let _ = client.beta().sites().drive().delta().by_id(ID);
    assert_url_id_equals_beta(&client, ID, DriveEndPoint::Delta);
    let _ = client.beta().sites().drive().shared_with_me().by_id(ID);
    assert_url_id_equals_beta(&client, ID, DriveEndPoint::SharedWithMe);
    let _ = client.beta().sites().drive().special_documents().by_id(ID);
    assert_url_id_equals_beta(&client, ID, DriveEndPoint::SpecialDocuments);
    let _ = client
        .beta()
        .sites()
        .drive()
        .special_documents_child()
        .by_id(ID);
    assert_url_id_equals_beta(&client, ID, DriveEndPoint::SpecialDocumentsChild);
    let _ = client.beta().sites().drive().special_photos().by_id(ID);
    assert_url_id_equals_beta(&client, ID, DriveEndPoint::SpecialPhotos);
    let _ = client
        .beta()
        .sites()
        .drive()
        .special_photos_child()
        .by_id(ID);
    assert_url_id_equals_beta(&client, ID, DriveEndPoint::SpecialPhotosChild);
    let _ = client
        .beta()
        .sites()
        .drive()
        .special_camera_roll()
        .by_id(ID);
    assert_url_id_equals_beta(&client, ID, DriveEndPoint::SpecialCameraRoll);
    let _ = client
        .beta()
        .sites()
        .drive()
        .special_camera_roll_child()
        .by_id(ID);
    assert_url_id_equals_beta(&client, ID, DriveEndPoint::SpecialCameraRollChild);
    let _ = client.beta().sites().drive().special_app_root().by_id(ID);
    assert_url_id_equals_beta(&client, ID, DriveEndPoint::SpecialAppRoot);
    let _ = client
        .beta()
        .sites()
        .drive()
        .special_app_root_child()
        .by_id(ID);
    assert_url_id_equals_beta(&client, ID, DriveEndPoint::SpecialAppRootChild);
    let _ = client.beta().sites().drive().special_music().by_id(ID);
    assert_url_id_equals_beta(&client, ID, DriveEndPoint::SpecialMusic);
    let _ = client
        .beta()
        .sites()
        .drive()
        .special_music_child()
        .by_id(ID);
    assert_url_id_equals_beta(&client, ID, DriveEndPoint::SpecialMusicChild);
}

#[test]
fn macro_endpoint_v2() {
    let client = get_drive();
    let _ = client.beta().me().drive().drive();
    assert_url_equals_beta(&client, DriveEndPoint::Drive);
    let _ = client.beta().me().drive().root();
    assert_url_equals_beta(&client, DriveEndPoint::DriveRoot);
    let _ = client.beta().me().drive().root_children();
    assert_url_equals_beta(&client, DriveEndPoint::DriveRootChild);
    let _ = client.beta().me().drive().recent();
    assert_url_equals_beta(&client, DriveEndPoint::DriveRecent);
    let _ = client.beta().me().drive().delta();
    assert_url_equals_beta(&client, DriveEndPoint::Delta);
    let _ = client.beta().me().drive().shared_with_me();
    assert_url_equals_beta(&client, DriveEndPoint::SharedWithMe);
    let _ = client.beta().me().drive().special_documents();
    assert_url_equals_beta(&client, DriveEndPoint::SpecialDocuments);
    let _ = client.beta().me().drive().special_documents_child();
    assert_url_equals_beta(&client, DriveEndPoint::SpecialDocumentsChild);
    let _ = client.beta().me().drive().special_photos();
    assert_url_equals_beta(&client, DriveEndPoint::SpecialPhotos);
    let _ = client.beta().me().drive().special_photos_child();
    assert_url_equals_beta(&client, DriveEndPoint::SpecialPhotosChild);
    let _ = client.beta().me().drive().special_camera_roll();
    assert_url_equals_beta(&client, DriveEndPoint::SpecialCameraRoll);
    let _ = client.beta().me().drive().special_camera_roll_child();
    assert_url_equals_beta(&client, DriveEndPoint::SpecialCameraRollChild);
    let _ = client.beta().me().drive().special_app_root();
    assert_url_equals_beta(&client, DriveEndPoint::SpecialAppRoot);
    let _ = client.beta().me().drive().special_app_root_child();
    assert_url_equals_beta(&client, DriveEndPoint::SpecialAppRootChild);
    let _ = client.beta().me().drive().special_music();
    assert_url_equals_beta(&client, DriveEndPoint::SpecialMusic);
    let _ = client.beta().me().drive().special_music_child();
    assert_url_equals_beta(&client, DriveEndPoint::SpecialMusicChild);
}

fn assert_url_eq(client: &Graph, path: &str) {
    client.url_ref(|url| {
        assert_eq!(vec![GRAPH_URL, path].join(""), url.to_string(),);
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
        .preview(None)
        .by_path("Documents/preview.txt");
    assert_url_eq(&client, "/me/drive/root:/Documents%2Fpreview.txt:/preview");

    let _ = client
        .v1()
        .drives()
        .drive()
        .preview(None)
        .by_path_id("32p99453", "Documents/preview.txt");
    assert_url_eq(
        &client,
        "/drives/32p99453/root:/Documents%2Fpreview.txt:/preview",
    );

    let _ = client
        .v1()
        .sites()
        .drive()
        .preview(None)
        .by_path_id("32p99453", "Documents/preview.txt");
    assert_url_eq(
        &client,
        "/sites/32p99453/drive/root:/Documents%2Fpreview.txt:/preview",
    );

    let _ = client
        .v1()
        .groups()
        .drive()
        .preview(None)
        .by_path_id("32p99453", "Documents/preview.txt");
    assert_url_eq(
        &client,
        "/groups/32p99453/drive/root:/Documents%2Fpreview.txt:/preview",
    );

    let _ = client
        .v1()
        .users()
        .drive()
        .preview(None)
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
pub fn drive_activities() {
    let client = get_drive();
    let _ = client.v1().me().lists().activities("32p99453");
    client.format_ord();
    assert_url_eq(&client, "/me/lists/32p99453/activities");

    let _ = client
        .v1()
        .drives()
        .lists()
        .activities("132534")
        .by_id("32p99453");
    assert_url_eq(&client, "/drives/32p99453/lists/132534/activities");

    let _ = client
        .v1()
        .sites()
        .lists()
        .activities("132534")
        .by_id("32p99453");
    assert_url_eq(&client, "/sites/32p99453/lists/132534/activities");
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
    client.format_ord();
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
    client.format_ord();
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
