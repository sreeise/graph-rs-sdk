use rust_onedrive::drive::DriveEndPoint;

pub fn main() {
    assert_eq!(
        DriveEndPoint::Drive.v1_url(),
        "https://graph.microsoft.com/v1.0/drive"
    );
    // Any endpoint can also use to_string().
    // Internally to_string() uses to_url().
    // ToString always uses the V1 host.
    assert_eq!(
        DriveEndPoint::Drive.to_string(),
        "https://graph.microsoft.com/v1.0/drive"
    );

    assert_eq!(
        DriveEndPoint::DriveMe.v1_url(),
        "https://graph.microsoft.com/v1.0/me/drive"
    );
    assert_eq!(
        DriveEndPoint::DriveRoot.v1_url(),
        "https://graph.microsoft.com/v1.0/drive/root"
    );
    assert_eq!(
        DriveEndPoint::DriveRootMe.v1_url(),
        "https://graph.microsoft.com/v1.0/me/drive/root"
    );
    assert_eq!(
        DriveEndPoint::DriveRootChild.v1_url(),
        "https://graph.microsoft.com/v1.0/drive/root/children"
    );

    // You can also use the beta Graph beta endpoint
    assert_eq!(
        DriveEndPoint::Drive.beta_url(),
        "https://graph.microsoft.com/beta/drive"
    );
    assert_eq!(
        DriveEndPoint::DriveMe.beta_url(),
        "https://graph.microsoft.com/beta/me/drive"
    );
    assert_eq!(
        DriveEndPoint::DriveRoot.beta_url(),
        "https://graph.microsoft.com/beta/drive/root"
    );
}
