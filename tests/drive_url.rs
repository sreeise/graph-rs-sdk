use rust_onedrive::drive::event::DriveEvent;
use rust_onedrive::drive::{DriveEndPoint, DriveResource, DriveVersion};
use rust_onedrive::prelude::DriveUrl;

#[test]
fn from_drive_end_point() {
    let mut url = DriveUrl::from(DriveVersion::V1);
    url.endpoint(DriveEndPoint::Drive);
    assert_eq!("https://graph.microsoft.com/v1.0/drive", url.as_ref());
}

#[test]
fn test_resource_path() {
    let mut drive_url = DriveUrl::new(DriveVersion::V1);
    drive_url.resource(DriveResource::Drives);
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives",
        drive_url.as_ref()
    );
}

#[test]
fn from_drive_end_point_drive_url() {
    let mut drive_url = DriveUrl::new(DriveVersion::V1);
    drive_url.endpoint(DriveEndPoint::Drive);
    assert_eq!("https://graph.microsoft.com/v1.0/drive", drive_url.as_ref());
}

#[test]
fn drive_event() {
    let mut drive_url = DriveUrl::new(DriveVersion::V1);
    drive_url
        .resource(DriveResource::Drives)
        .extend_path(&["3232093-2320-2387429378", "items", "1234"])
        .event(DriveEvent::Download);
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/3232093-2320-2387429378/items/1234/content",
        drive_url.as_ref(),
    );
}

#[test]
fn drive_path() {
    let mut drive_url = DriveUrl::new(DriveVersion::V1);
    drive_url
        .resource(DriveResource::Drives)
        .extend_path(&["1234:", "example.txt:", "content"]);
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/1234:/example.txt:/content",
        drive_url.as_ref(),
    );
}
