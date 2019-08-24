use rust_onedrive::drive::endpoint::DriveEndPoint;
use rust_onedrive::drive::DriveVersion;
use rust_onedrive::prelude::DriveUrl;

#[test]
fn from_drive_end_point() {
    let mut url = DriveUrl::from(DriveVersion::V1);
    url.endpoint(DriveEndPoint::Drive);
    assert_eq!("https://graph.microsoft.com/v1.0/drive", url.as_ref());
}

#[test]
fn from_drive_end_point_drive_url() {
    let mut drive_url = DriveUrl::new(DriveVersion::V2);
    drive_url.endpoint(DriveEndPoint::Drive);
    assert_eq!("https://graph.microsoft.com/beta/drive", drive_url.as_ref());
}
