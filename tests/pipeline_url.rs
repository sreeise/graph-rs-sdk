use rust_onedrive::drive::event::{ConflictBehavior, NewFolder};
use rust_onedrive::drive::itemreference::ItemReference;
use rust_onedrive::prelude::*;

fn get_drive() -> Drive {
    Drive::new("")
}

#[test]
fn query_mutate() {
    let mut pipeline = get_drive().v1().drive();
    pipeline.select(&["name"]);
    pipeline.top("3");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drive?select=name&top=3",
        url.as_ref()
    );

    let mut pipeline = get_drive().v1().drive_root();
    pipeline.expand(&["children"]);
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drive/root?expand=children",
        url.as_ref()
    );
}

#[test]
fn event_copy_url() {
    let item_ref = ItemReference::default();
    let pipeline = get_drive().v1().me().copy("3", &item_ref, None);
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/3/copy",
        url.as_str()
    );

    let pipeline = get_drive().v1().drives().copy("3", "4", &item_ref, None);

    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/4/items/3/copy",
        url.as_str()
    );

    let pipeline = get_drive().v1().sites().copy("3", "4", &item_ref, None);
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/4/drive/items/3/copy",
        url.as_str()
    );

    let pipeline = get_drive().v1().groups().copy("3", "4", &item_ref, None);
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/4/drive/items/3/copy",
        url.as_str()
    );

    let pipeline = get_drive().v1().users().copy("3", "4", &item_ref, None);
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/4/drive/items/3/copy",
        url.as_str()
    );
}

#[test]
fn event_create_folder() {
    let pipeline = get_drive()
        .v1()
        .me()
        .create_folder("132425", NewFolder::new("name", ConflictBehavior::Replace));
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/132425/children",
        url.as_str()
    );

    let pipeline = get_drive().v1().drives().create_folder(
        "4",
        "132425",
        NewFolder::new("name", ConflictBehavior::Replace),
    );
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/4/items/132425/children",
        url.as_str()
    );

    let pipeline = get_drive().v1().sites().create_folder(
        "4",
        "132425",
        NewFolder::new("name", ConflictBehavior::Replace),
    );
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/4/drive/items/132425/children",
        url.as_str()
    );

    let pipeline = get_drive().v1().groups().create_folder(
        "4",
        "132425",
        NewFolder::new("name", ConflictBehavior::Replace),
    );
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/4/drive/items/132425/children",
        url.as_str()
    );

    let pipeline = get_drive().v1().users().create_folder(
        "4",
        "132425",
        NewFolder::new("name", ConflictBehavior::Replace),
    );
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/4/drive/items/132425/children",
        url.as_str()
    );
}

#[test]
fn event_delete() {
    let pipeline = get_drive().v1().me().delete("32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/32p99453",
        url.as_str()
    );

    let pipeline = get_drive().v1().drives().delete("32p99453", "132534");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/132534/items/32p99453",
        url.as_str()
    );

    let pipeline = get_drive().v1().sites().delete("32p99453", "132534");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/132534/drive/items/32p99453",
        url.as_str()
    );

    let pipeline = get_drive().v1().groups().delete("32p99453", "132534");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/132534/drive/items/32p99453",
        url.as_str()
    );

    let pipeline = get_drive().v1().users().delete("32p99453", "132534");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/132534/drive/items/32p99453",
        url.as_str()
    );
}

#[test]
fn event_get_item() {
    let pipeline = get_drive().v1().me().get_item("32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/32p99453",
        url.as_str()
    );

    let pipeline = get_drive().v1().drives().get_item("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534",
        url.as_str()
    );

    let pipeline = get_drive().v1().sites().get_item("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534",
        url.as_str()
    );

    let pipeline = get_drive().v1().groups().get_item("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534",
        url.as_str()
    );

    let pipeline = get_drive().v1().users().get_item("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/132534",
        url.as_str()
    );
}

#[test]
fn event_thumbnails() {
    let pipeline = get_drive().v1().me().thumbnails("32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/32p99453/thumbnails",
        url.as_str()
    );

    let pipeline = get_drive().v1().drives().thumbnails("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534/thumbnails",
        url.as_str()
    );

    let pipeline = get_drive().v1().sites().thumbnails("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534/thumbnails",
        url.as_str()
    );

    let pipeline = get_drive().v1().groups().thumbnails("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534/thumbnails",
        url.as_str()
    );

    let pipeline = get_drive().v1().users().thumbnails("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/132534/thumbnails",
        url.as_str()
    );
}

#[test]
fn event_single_thumbnail() {
    let pipeline = get_drive().v1().me().single_thumbnail("132534", "4", "100");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/132534/thumbnails/4/100",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .single_thumbnail("132534", "32p99453", "4", "100");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534/thumbnails/4/100",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .single_thumbnail("132534", "32p99453", "4", "100");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534/thumbnails/4/100",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .single_thumbnail("132534", "32p99453", "4", "100");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534/thumbnails/4/100",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .single_thumbnail("132534", "32p99453", "4", "100");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/132534/thumbnails/4/100",
        url.as_str()
    );
}

#[test]
fn event_thumbnail_binary() {
    let pipeline = get_drive().v1().me().thumbnail_binary("132534", "4", "100");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/132534/thumbnails/4/100/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .thumbnail_binary("132534", "32p99453", "4", "100");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534/thumbnails/4/100/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .thumbnail_binary("132534", "32p99453", "4", "100");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534/thumbnails/4/100/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .thumbnail_binary("132534", "32p99453", "4", "100");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534/thumbnails/4/100/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .thumbnail_binary("132534", "32p99453", "4", "100");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/132534/thumbnails/4/100/content",
        url.as_str()
    );
}

#[test]
fn event_update() {
    let drive_item = DriveItem::default();
    let pipeline = get_drive().v1().me().update("32p99453", &drive_item);
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/32p99453",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .update("132534", "32p99453", &drive_item);
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .update("132534", "32p99453", &drive_item);
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .update("132534", "32p99453", &drive_item);
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .update("132534", "32p99453", &drive_item);
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/132534",
        url.as_str()
    );
}

#[test]
pub fn event_upload_new() {
    let pipeline = get_drive()
        .v1()
        .me()
        .upload_new("132534", "./test_files/item_tes/drive_info.json")
        .unwrap();
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/132534:/drive_info.json:/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .upload_new(
            "132534",
            "32p99453",
            "./test_files/item_test/drive_info.json",
        )
        .unwrap();
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534:/drive_info.json:/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .upload_new(
            "132534",
            "32p99453",
            "./test_files/item_test/drive_info.json",
        )
        .unwrap();
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534:/drive_info.json:/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .upload_new(
            "132534",
            "32p99453",
            "./test_files/item_test/drive_info.json",
        )
        .unwrap();
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534:/drive_info.json:/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .upload_new(
            "132534",
            "32p99453",
            "./test_files/item_test/drive_info.json",
        )
        .unwrap();
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/132534:/drive_info.json:/content",
        url.as_str()
    );
}

#[test]
pub fn event_upload_replace() {
    let pipeline = get_drive()
        .v1()
        .me()
        .upload_replace("132534", "./test_files/item_tes/drive_info.json");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/132534/content",
        url.as_str()
    );

    let pipeline = get_drive().v1().drives().upload_replace(
        "132534",
        "32p99453",
        "./test_files/item_test/drive_info.json",
    );
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534/content",
        url.as_str()
    );

    let pipeline = get_drive().v1().sites().upload_replace(
        "132534",
        "32p99453",
        "./test_files/item_test/drive_info.json",
    );
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534/content",
        url.as_str()
    );

    let pipeline = get_drive().v1().groups().upload_replace(
        "132534",
        "32p99453",
        "./test_files/item_test/drive_info.json",
    );
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534/content",
        url.as_str()
    );

    let pipeline = get_drive().v1().users().upload_replace(
        "132534",
        "32p99453",
        "./test_files/item_test/drive_info.json",
    );
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/132534/content",
        url.as_str()
    );
}
