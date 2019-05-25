#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

pub use from_to_file::*;
use rocket::http::Status;
use rocket::local::Client;
use rocket::Rocket;
use rocket_codegen::routes;
use rust_onedrive::drive::driveinfo::DriveInfo;
use rust_onedrive::drive::driveitem::DriveItem;
use rust_onedrive::drive::filesysteminfo::FileSystemInfo;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;

fn file_to_string(path: &str) -> String {
    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");
    contents
}

#[get("/v1.0/drive")]
fn drive() -> String {
    file_to_string("test_files/item_test/drive_info.json")
}

#[get("/v1.0/me/drive/root/children")]
fn drive_root() -> String {
    file_to_string("test_files/item_test/drive_root_children.json")
}

#[get("/v1.0/me/drive/recent")]
fn drive_recent() -> String {
    file_to_string("test_files/item_test/drive_recent.json")
}

fn rocket() -> Rocket {
    rocket::ignite().mount("/", routes![drive, drive_root, drive_recent])
}

fn main() {
    rocket().launch();
}

fn rocket_request_drive_item(request: &str) -> DriveItem {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get(request).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let drive_item: DriveItem = DriveItem::try_from(response.body_string().unwrap()).unwrap();
    drive_item
}

fn rocket_request_drive_info(request: &str) -> DriveInfo {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get(request).dispatch();
    assert_eq!(response.status(), Status::Ok);
    let drive_info: DriveInfo = DriveInfo::try_from(response.body_string().unwrap()).unwrap();
    drive_info
}

#[test]
fn drive_info_item() {
    let drive_info = rocket_request_drive_info("/v1.0/drive");
    assert_eq!(
        "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI",
        drive_info.id().unwrap()
    );
}

#[test]
fn drive_root_item() {
    let drive_item = rocket_request_drive_item("/v1.0/me/drive/root/children");
    assert_eq!(drive_item.data_context(), Some("https://graph.microsoft.com/v1.0/$metadata#users('48d31887-5fad-4d73-a9f5-3c356e68a038')/drive/root/children".into()));
    assert_eq!(
        drive_item.value_idx(1).created_date_time(),
        Some("2017-08-07T16:16:30Z".into())
    );
    assert_eq!(
        drive_item.value_idx(2).created_date_time(),
        Some("2017-08-07T16:10:22Z".into())
    );

    let file_system_info = FileSystemInfo::new(
        Some("2017-09-13T21:51:28Z".into()),
        Some("2017-09-13T21:51:28Z".into()),
    );
    assert_eq!(
        drive_item
            .value()
            .unwrap()
            .last()
            .unwrap()
            .file_system_info(),
        Some(file_system_info)
    );

    let drive_item_from_json_file: DriveItem =
        DriveItem::from_json_file("test_files/item_test/drive_root_children.json").unwrap();
    assert_eq!(drive_item_from_json_file, drive_item);
}

#[test]
fn drive_recent_item() {
    let drive_item = rocket_request_drive_item("/v1.0/me/drive/recent");
    assert_eq!(drive_item.value_idx(1).web_url(), Some("https://m365x214355-my.sharepoint.com/personal/meganb_m365x214355_onmicrosoft_com/_layouts/15/Doc.aspx?sourcedoc=%7B2964723D-9E45-470E-8FE4-85CEDA9D4018%7D&file=Carbon%20Deposits%20Analysis.xlsx&action=default&mobileredirect=true&DefaultItemOpen=1".into()));
}
