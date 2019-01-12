use rust_onedrive::drive::endpoint::DriveEndPoint;

pub fn main() {
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
}
