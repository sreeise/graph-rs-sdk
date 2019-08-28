use rust_onedrive::drive::drive_item::itemreference::ItemReference;
use rust_onedrive::drive::event::{ConflictBehavior, NewFolder};
use rust_onedrive::prelude::*;
use std::ffi::OsString;

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
fn event_copy() {
    let item_ref = ItemReference::default();
    let pipeline = get_drive().v1().me().copy(None, &item_ref).by_id("3");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/3/copy",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .copy(None, &item_ref)
        .by_id("3", "4");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/4/items/3/copy",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .copy(None, &item_ref)
        .by_id("3", "4");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/4/drive/items/3/copy",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .copy(None, &item_ref)
        .by_id("3", "4");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/4/drive/items/3/copy",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .copy(None, &item_ref)
        .by_id("3", "4");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/4/drive/items/3/copy",
        url.as_str()
    );
}

#[test]
fn event_copy_path() {
    let path = OsString::from("Documents/item.txt");
    let item_ref = ItemReference::default();

    let pipeline = get_drive()
        .v1()
        .me()
        .copy(None, &item_ref)
        .by_path(path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/root:/Documents%2Fitem.txt:/copy",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .copy(None, &item_ref)
        .by_path("132534", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/132534/root:/Documents%2Fitem.txt:/copy",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .copy(None, &item_ref)
        .by_path("132534", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/132534/drive/root:/Documents%2Fitem.txt:/copy",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .copy(None, &item_ref)
        .by_path("132534", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/132534/drive/root:/Documents%2Fitem.txt:/copy",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .copy(None, &item_ref)
        .by_path("132534", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/132534/drive/root:/Documents%2Fitem.txt:/copy",
        url.as_str()
    );
}

#[test]
fn event_create_folder() {
    let pipeline = get_drive()
        .v1()
        .me()
        .create_folder(NewFolder::new("name", ConflictBehavior::Replace))
        .by_id("132425");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/132425/children",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .create_folder(NewFolder::new("name", ConflictBehavior::Replace))
        .by_id("132425", "4");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/4/items/132425/children",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .create_folder(NewFolder::new("name", ConflictBehavior::Replace))
        .by_id("132425", "4");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/4/drive/items/132425/children",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .create_folder(NewFolder::new("name", ConflictBehavior::Replace))
        .by_id("132425", "4");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/4/drive/items/132425/children",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .create_folder(NewFolder::new("name", ConflictBehavior::Replace))
        .by_id("132425", "4");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/4/drive/items/132425/children",
        url.as_str()
    );
}

#[test]
fn event_create_folder_path() {
    let path = OsString::from("Documents");

    let pipeline = get_drive()
        .v1()
        .me()
        .create_folder(NewFolder::new("name", ConflictBehavior::Replace))
        .by_path(path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/root:/Documents:/children",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .create_folder(NewFolder::new("name", ConflictBehavior::Replace))
        .by_path("132534", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/132534/root:/Documents:/children",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .create_folder(NewFolder::new("name", ConflictBehavior::Replace))
        .by_path("132534", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/132534/drive/root:/Documents:/children",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .create_folder(NewFolder::new("name", ConflictBehavior::Replace))
        .by_path("132534", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/132534/drive/root:/Documents:/children",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .create_folder(NewFolder::new("name", ConflictBehavior::Replace))
        .by_path("132534", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/132534/drive/root:/Documents:/children",
        url.as_str()
    );
}

#[test]
fn event_delete() {
    let pipeline = get_drive().v1().me().delete().by_id("32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/32p99453",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .delete()
        .by_id("32p99453", "132534");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/132534/items/32p99453",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .delete()
        .by_id("32p99453", "132534");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/132534/drive/items/32p99453",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .delete()
        .by_id("32p99453", "132534");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/132534/drive/items/32p99453",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .delete()
        .by_id("32p99453", "132534");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/132534/drive/items/32p99453",
        url.as_str()
    );
}

#[test]
fn event_delete_path() {
    let path = OsString::from("Documents/item.txt");

    let pipeline = get_drive().v1().me().delete().by_path(path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/root:/Documents%2Fitem.txt",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .delete()
        .by_path("132534", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/132534/root:/Documents%2Fitem.txt",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .delete()
        .by_path("132534", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/132534/drive/root:/Documents%2Fitem.txt",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .delete()
        .by_path("132534", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/132534/drive/root:/Documents%2Fitem.txt",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .delete()
        .by_path("132534", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/132534/drive/root:/Documents%2Fitem.txt",
        url.as_str()
    );
}

#[test]
fn event_get_item() {
    let pipeline = get_drive().v1().me().get_item().by_id("32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/32p99453",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .get_item()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .get_item()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .get_item()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .get_item()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/132534",
        url.as_str()
    );
}

#[test]
fn event_get_item_path() {
    let path = OsString::from("Documents/item.txt");

    let pipeline = get_drive().v1().me().get_item().by_path(path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/root:/Documents%2Fitem.txt",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .get_item()
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/root:/Documents%2Fitem.txt",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .get_item()
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/root:/Documents%2Fitem.txt",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .get_item()
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/root:/Documents%2Fitem.txt",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .get_item()
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/root:/Documents%2Fitem.txt",
        url.as_str()
    );
}

#[test]
fn event_thumbnails() {
    let pipeline = get_drive().v1().me().thumbnails().by_id("32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/32p99453/thumbnails",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .thumbnails()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534/thumbnails",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .thumbnails()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534/thumbnails",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .thumbnails()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534/thumbnails",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .thumbnails()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/132534/thumbnails",
        url.as_str()
    );
}

#[test]
fn event_single_thumbnail() {
    let pipeline = get_drive()
        .v1()
        .me()
        .single_thumbnail("4", "100")
        .by_id("132534");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/132534/thumbnails/4/100",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .single_thumbnail("4", "100")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534/thumbnails/4/100",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .single_thumbnail("4", "100")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534/thumbnails/4/100",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .single_thumbnail("4", "100")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534/thumbnails/4/100",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .single_thumbnail("4", "100")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/132534/thumbnails/4/100",
        url.as_str()
    );
}

#[test]
fn event_thumbnail_binary() {
    let pipeline = get_drive()
        .v1()
        .me()
        .thumbnail_binary("4", "100")
        .by_id("132534");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/132534/thumbnails/4/100/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .thumbnail_binary("4", "100")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534/thumbnails/4/100/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .thumbnail_binary("4", "100")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534/thumbnails/4/100/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .thumbnail_binary("4", "100")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534/thumbnails/4/100/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .thumbnail_binary("4", "100")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/132534/thumbnails/4/100/content",
        url.as_str()
    );
}

#[test]
fn event_update() {
    let drive_item = DriveItem::default();
    let pipeline = get_drive().v1().me().update(&drive_item).by_id("32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/32p99453",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .update(&drive_item)
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .update(&drive_item)
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .update(&drive_item)
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .update(&drive_item)
        .by_id("132534", "32p99453");
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
        .upload_new("./test_files/item_tes/drive_info.json")
        .unwrap()
        .by_id("132534");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/132534:/drive_info.json:/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534:/drive_info.json:/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534:/drive_info.json:/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534:/drive_info.json:/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/132534:/drive_info.json:/content",
        url.as_str()
    );
}

#[test]
pub fn event_upload_new_path() {
    let path = OsString::from("Documents/drive_info.json");

    let pipeline = get_drive()
        .v1()
        .me()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_path(path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/root:/Documents%2Fdrive_info.json:/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/root:/Documents%2Fdrive_info.json:/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/root:/Documents%2Fdrive_info.json:/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/root:/Documents%2Fdrive_info.json:/content",
        url.as_str()
    );
    let pipeline = get_drive()
        .v1()
        .users()
        .upload_new("./test_files/item_test/drive_info.json")
        .unwrap()
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/root:/Documents%2Fdrive_info.json:/content",
        url.as_str()
    );
}

#[test]
pub fn event_upload_replace() {
    let pipeline = get_drive()
        .v1()
        .me()
        .upload_replace("./test_files/item_tes/drive_info.json")
        .by_id("132534");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/132534/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .upload_replace("./test_files/item_test/drive_info.json")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .upload_replace("./test_files/item_test/drive_info.json")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .upload_replace("./test_files/item_test/drive_info.json")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .upload_replace("./test_files/item_test/drive_info.json")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/132534/content",
        url.as_str()
    );
}

#[test]
pub fn event_list_versions() {
    let pipeline = get_drive().v1().me().list_versions().by_id("132534");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/132534/versions",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .list_versions()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534/versions",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .list_versions()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534/versions",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .list_versions()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534/versions",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .list_versions()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/132534/versions",
        url.as_str()
    );
}

#[test]
pub fn event_list_versions_path() {
    let path = OsString::from("Documents/item.txt");

    let pipeline = get_drive().v1().me().list_versions().by_path(path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/root:/Documents%2Fitem.txt:/versions",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .list_versions()
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/root:/Documents%2Fitem.txt:/versions",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .list_versions()
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/root:/Documents%2Fitem.txt:/versions",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .list_versions()
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/root:/Documents%2Fitem.txt:/versions",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .list_versions()
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/root:/Documents%2Fitem.txt:/versions",
        url.as_str()
    );
}

#[test]
pub fn event_restore_version() {
    let pipeline = get_drive()
        .v1()
        .me()
        .restore_version("34492566a")
        .by_id("132534");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/132534/versions/34492566a/restoreVersion",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .restore_version("34492566a")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534/versions/34492566a/restoreVersion",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .restore_version("34492566a")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534/versions/34492566a/restoreVersion",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .restore_version("34492566a")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534/versions/34492566a/restoreVersion",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .restore_version("34492566a")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/132534/versions/34492566a/restoreVersion",
        url.as_str()
    );
}

#[test]
pub fn event_restore_version_path() {
    let path = OsString::from("Documents/item.txt");
    let pipeline = get_drive()
        .v1()
        .me()
        .restore_version("34492566a")
        .by_path(path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/root:/Documents%2Fitem.txt:/versions/34492566a/restoreVersion",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .restore_version("34492566a")
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/root:/Documents%2Fitem.txt:/versions/34492566a/restoreVersion",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .restore_version("34492566a")
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/root:/Documents%2Fitem.txt:/versions/34492566a/restoreVersion",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .restore_version("34492566a")
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/root:/Documents%2Fitem.txt:/versions/34492566a/restoreVersion",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .restore_version("34492566a")
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/root:/Documents%2Fitem.txt:/versions/34492566a/restoreVersion",
        url.as_str()
    );
}

#[test]
pub fn event_list_activities() {
    let pipeline = get_drive().v1().me().list_drive_activities();
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/activities",
        url.as_str()
    );

    let pipeline = get_drive().v1().drives().list_drive_activities("32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/activities",
        url.as_str()
    );

    let pipeline = get_drive().v1().sites().list_drive_activities("32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/activities",
        url.as_str()
    );

    let pipeline = get_drive().v1().groups().list_drive_activities("32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/activities",
        url.as_str()
    );

    let pipeline = get_drive().v1().users().list_drive_activities("32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/activities",
        url.as_str()
    );
}

#[test]
pub fn event_list_item_activities() {
    let pipeline = get_drive().v1().me().list_item_activities().by_id("132534");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/132534/activities",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .list_item_activities()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/132534/activities",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .list_item_activities()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/132534/activities",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .list_item_activities()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/132534/activities",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .list_item_activities()
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/132534/activities",
        url.as_str()
    );
}

#[test]
pub fn event_activities_from_list_item() {
    let pipeline = get_drive()
        .v1()
        .me()
        .activities_from_list_item("30ad9832")
        .by_id("132534");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/lists/30ad9832/items/132534/activities",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .activities_from_list_item("30ad9832")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/lists/30ad9832/items/132534/activities",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .activities_from_list_item("30ad9832")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/lists/30ad9832/items/132534/activities",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .activities_from_list_item("30ad9832")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/lists/30ad9832/items/132534/activities",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .activities_from_list_item("30ad9832")
        .by_id("132534", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/lists/30ad9832/items/132534/activities",
        url.as_str()
    );
}

#[test]
pub fn event_upload_session_replace() {
    let mut path = OsString::new();
    path.push("Documents/complete_drive_item.json");
    let mut file = OsString::new();
    file.push("./test_files/item_test/complete_drive_item.json");

    let pipeline = get_drive()
        .v1()
        .me()
        .upload_session_replace(file.clone())
        .by_id("3dj309v");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/3dj309v/createUploadSession",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .upload_session_replace(file.clone())
        .by_id("3dj309v", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/3dj309v/createUploadSession",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .upload_session_replace(file.clone())
        .by_id("3dj309v", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/3dj309v/createUploadSession",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .upload_session_replace(file.clone())
        .by_id("3dj309v", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/3dj309v/createUploadSession",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .upload_session_replace(file.clone())
        .by_id("3dj309v", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/3dj309v/createUploadSession",
        url.as_str()
    );
}

#[test]
pub fn event_upload_session_new() {
    let mut path = OsString::new();
    path.push("Documents/complete_drive_item.json");

    let mut file = OsString::new();
    file.push("./test_files/item_test/complete_drive_item.json");
    let pipeline = get_drive()
        .v1()
        .me()
        .upload_session_new("./test_files/item_test/complete_drive_item.json", None)
        .by_path(path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/root:/Documents%2Fcomplete_drive_item.json:/createUploadSession",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .upload_session_new(file.clone(), None)
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/root:/Documents%2Fcomplete_drive_item.json:/createUploadSession",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .upload_session_new(file.clone(), None)
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/root:/Documents%2Fcomplete_drive_item.json:/createUploadSession",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .upload_session_new(file.clone(), None)
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/root:/Documents%2Fcomplete_drive_item.json:/createUploadSession",
        url.as_str()
    );
    let pipeline = get_drive()
        .v1()
        .users()
        .upload_session_new(file.clone(), None)
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/root:/Documents%2Fcomplete_drive_item.json:/createUploadSession",
        url.as_str()
    );
}

#[test]
pub fn event_preview() {
    let pipeline = get_drive().v1().me().preview(None).by_id("3dj309v");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/3dj309v/preview",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .preview(None)
        .by_id("3dj309v", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/3dj309v/preview",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .preview(None)
        .by_id("3dj309v", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/3dj309v/preview",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .preview(None)
        .by_id("3dj309v", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/3dj309v/preview",
        url.as_str()
    );
    let pipeline = get_drive()
        .v1()
        .users()
        .preview(None)
        .by_id("3dj309v", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/3dj309v/preview",
        url.as_str()
    );
}

#[test]
pub fn event_preview_path() {
    let path = OsString::from("Documents/preview.txt");
    let pipeline = get_drive().v1().me().preview(None).by_path(path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/root:/Documents%2Fpreview.txt:/preview",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .preview(None)
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/root:/Documents%2Fpreview.txt:/preview",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .preview(None)
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/root:/Documents%2Fpreview.txt:/preview",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .preview(None)
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/root:/Documents%2Fpreview.txt:/preview",
        url.as_str()
    );
    let pipeline = get_drive()
        .v1()
        .users()
        .preview(None)
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/root:/Documents%2Fpreview.txt:/preview",
        url.as_str()
    );
}

#[test]
fn event_download() {
    let pipeline = get_drive()
        .v1()
        .me()
        .download("./example_files/examples")
        .by_id("1234");

    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/1234/content",
        url.as_str(),
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .download("./example_files/examples")
        .by_id("1234", "32p99453");

    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/items/1234/content",
        url.as_str(),
    );
    let pipeline = get_drive()
        .v1()
        .sites()
        .download("./example_files/examples")
        .by_id("1234", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/items/1234/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .download("./example_files/examples")
        .by_id("1234", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/items/1234/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .download("./example_files/examples")
        .by_id("1234", "32p99453");
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/items/1234/content",
        url.as_str()
    );
}

#[test]
fn event_download_path() {
    let path = OsString::from("Documents/preview.txt");

    let pipeline = get_drive()
        .v1()
        .me()
        .download("./example_files/examples")
        .by_path(path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/root:/Documents%2Fpreview.txt:/content",
        url.as_str(),
    );

    let pipeline = get_drive()
        .v1()
        .drives()
        .download("./example_files/examples")
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/32p99453/root:/Documents%2Fpreview.txt:/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .sites()
        .download("./example_files/examples")
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/32p99453/drive/root:/Documents%2Fpreview.txt:/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .groups()
        .download("./example_files/examples")
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/32p99453/drive/root:/Documents%2Fpreview.txt:/content",
        url.as_str()
    );

    let pipeline = get_drive()
        .v1()
        .users()
        .download("./example_files/examples")
        .by_path("32p99453", path.clone());
    let url: &DriveUrl = pipeline.as_ref();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/32p99453/drive/root:/Documents%2Fpreview.txt:/content",
        url.as_str()
    );
}
