use rust_onedrive::drive::endpoint::DriveEndPoint;

pub fn main() {
    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::Drive),
        "https://graph.microsoft.com/v1.0/drive"
    );

    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::DriveMe),
        "https://graph.microsoft.com/v1.0/me/drive"
    );

    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::DriveRoot),
        "https://graph.microsoft.com/v1.0/drive/root"
    );

    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::DriveRootMe),
        "https://graph.microsoft.com/v1.0/me/drive/root"
    );

    assert_eq!(
        DriveEndPoint::build(DriveEndPoint::DriveRootChild),
        "https://graph.microsoft.com/v1.0/drive/root/children"
    );
}
