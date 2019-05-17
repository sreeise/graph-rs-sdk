use rust_onedrive::drive::{
    DriveEndPoint, DriveEvent, DriveEventPath, DriveResource, DriveVersion, PathBuilder,
};

#[test]
fn test_path_builder_host() {
    let mut pb = PathBuilder::new("http://login.microsoft.com");
    assert_eq!("http://login.microsoft.com", pb.build());
}

#[test]
#[should_panic]
fn test_path_builder_no_base() {
    PathBuilder::new("login.microsoft.com");
}

#[test]
fn test_resource_path() {
    let mut pb = PathBuilder::new("http://login.microsoft.com");
    pb.drive_resource(DriveResource::Drives);
    assert_eq!("http://login.microsoft.com/drives", pb.build())
}

#[test]
fn from_drive_end_point() {
    let mut p1 = PathBuilder::from(DriveEndPoint::Drive);
    assert_eq!("https://graph.microsoft.com/v1.0/drive", p1.build());

    let mut p2 = PathBuilder::new("https://graph.microsoft.com");
    p2.drive_endpoint(DriveEndPoint::Drive);
    assert_eq!("https://graph.microsoft.com/v1.0/drive", p1.build());
}

#[test]
fn from_drive_version() {
    let mut path = PathBuilder::from(DriveVersion::V1);
    assert_eq!("https://graph.microsoft.com/v1.0", path.build());
}

#[test]
fn from_drive_event_path() {
    let event_path = DriveEventPath::new(
        DriveVersion::V1,
        DriveResource::Drives,
        DriveEvent::Download,
        "1234",
    );
    let mut pb: PathBuilder = event_path.into();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drive/items/1234/content",
        pb.build()
    );

    let mut event_path2 = DriveEventPath::new(
        DriveVersion::V1,
        DriveResource::Drives,
        DriveEvent::Download,
        "1234",
    );
    event_path2.drive_id("3232093-2320-2387429378");
    let mut pb2: PathBuilder = event_path2.into();
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/3232093-2320-2387429378/items/1234/content",
        pb2.build()
    );
}
