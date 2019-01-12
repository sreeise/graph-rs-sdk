use rust_onedrive::drive::endpoint::DriveEndPoint;

#[test]
fn drive_endpoint_test() {
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::Drive).expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/drive"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::DriveMe).expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/me/drive"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::DriveRoot).expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/drive/root"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::DriveRootMe).expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/me/drive/root"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::DriveRootChild)
            .expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/drive/root/children"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SharedWithMe).expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/me/drive/sharedWithMe"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::DriveRecent).expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/me/drive/recent"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialDocuments)
            .expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/me/drive/special/documents"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialDocumentsChild)
            .expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/me/drive/special/documents/children"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialPhotos).expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/me/drive/special/photos"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialPhotosChild)
            .expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/me/special/photos/children"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialCameraRoll)
            .expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/me/drive/special/cameraroll"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialCameraRollChild)
            .expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/me/drive/special/cameraroll/children"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialAppRoot)
            .expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/me/drive/special/approot"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialAppRootChild)
            .expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/me/drive/special/approot/children"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialMusic).expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/me/drive/special/music"
    );
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::SpecialMusicChild)
            .expect("could not build drive endpoint"),
        "https://graph.microsoft.com/v1.0/me/drive/special/music/children"
    );
}
