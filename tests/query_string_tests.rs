use rust_onedrive::drive::query_string::QueryString;
use rust_onedrive::drive::DriveEndPoint;
use rust_onedrive::drive::{Drive, DriveVersion};

#[test]
fn query_string_tests() {
    let drive = Drive::new("", DriveVersion::V1);
    assert_eq!(
        drive.count_url(DriveEndPoint::DriveRootChild, "2"),
        "https://graph.microsoft.com/v1.0/drive/root/children?$count=2"
    );
    let vec = vec!["name", "size"];
    assert_eq!(
        drive.select_url(DriveEndPoint::Drive, &vec),
        "https://graph.microsoft.com/v1.0/drive?$select=name,size"
    );
    assert_eq!(
        drive.expand_url(DriveEndPoint::Drive, "children"),
        "https://graph.microsoft.com/v1.0/drive?$expand=children"
    );
    assert_eq!(
        drive.order_by_url(DriveEndPoint::Drive, "name"),
        "https://graph.microsoft.com/v1.0/drive?$orderby=name"
    );
    assert_eq!(
        drive.format_url(DriveEndPoint::Drive, "json"),
        "https://graph.microsoft.com/v1.0/drive?$format=json"
    );
    let filter_vec = vec!["Subject", "eq", "'Welcome'"];
    assert_eq!(
        drive.filter_url(DriveEndPoint::Drive, &filter_vec),
        "https://graph.microsoft.com/v1.0/drive?$filter=Subject eq 'Welcome'"
    );
    assert_eq!(
        drive.search_url(DriveEndPoint::Drive, "pizza"),
        "https://graph.microsoft.com/v1.0/drive?$search=pizza"
    );
    assert_eq!(
        drive.top_url(DriveEndPoint::Drive, "2"),
        "https://graph.microsoft.com/v1.0/drive?$top=2"
    );
    assert_eq!(
        drive.skip_url(DriveEndPoint::Drive, "2"),
        "https://graph.microsoft.com/v1.0/drive?$skip=2"
    );
}
