use rust_onedrive::drive::DriveResource;
use rust_onedrive::drive::{DownloadFormat, DriveEvent};

#[test]
fn drive_action_url() {
    let check_in_url = DriveResource::Drives.drive_item_resource(
        "{drive-id}",
        "{parent-item-id}",
        DriveEvent::CheckIn,
    );
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/{drive-id}/items/{parent-item-id}/checkin",
        check_in_url.as_str()
    );

    let check_out_url = DriveResource::Drives.drive_item_resource(
        "{drive-id}",
        "{parent-item-id}",
        DriveEvent::CheckOut,
    );
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/{drive-id}/items/{parent-item-id}/checkout",
        check_out_url.as_str()
    );
}

#[test]
fn groups_action_url() {
    let check_in_url = DriveResource::Groups.drive_item_resource(
        "{drive-id}",
        "{parent-item-id}",
        DriveEvent::CheckIn,
    );
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/{drive-id}/drive/items/{parent-item-id}/checkin",
        check_in_url.as_str()
    );

    let check_out_url = DriveResource::Groups.drive_item_resource(
        "{drive-id}",
        "{parent-item-id}",
        DriveEvent::CheckOut,
    );
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/{drive-id}/drive/items/{parent-item-id}/checkout",
        check_out_url.as_str()
    );
}

#[test]
fn sites_action_url() {
    let check_in_url = DriveResource::Sites.drive_item_resource(
        "{drive-id}",
        "{parent-item-id}",
        DriveEvent::CheckIn,
    );
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/{drive-id}/drive/items/{parent-item-id}/checkin",
        check_in_url.as_str()
    );

    let check_out_url = DriveResource::Sites.drive_item_resource(
        "{drive-id}",
        "{parent-item-id}",
        DriveEvent::CheckOut,
    );
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/{drive-id}/drive/items/{parent-item-id}/checkout",
        check_out_url.as_str()
    );
}

#[test]
fn users_action_url() {
    let check_in_url = DriveResource::Users.drive_item_resource(
        "{drive-id}",
        "{parent-item-id}",
        DriveEvent::CheckIn,
    );
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/{drive-id}/drive/items/{parent-item-id}/checkin",
        check_in_url.as_str()
    );

    let check_out_url = DriveResource::Users.drive_item_resource(
        "{drive-id}",
        "{parent-item-id}",
        DriveEvent::CheckOut,
    );
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/{drive-id}/drive/items/{parent-item-id}/checkout",
        check_out_url.as_str()
    );
}

#[test]
fn me_action_url() {
    let check_in_url = DriveResource::Me.item_resource("{parent-item-id}", DriveEvent::CheckIn);
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/{parent-item-id}/checkin",
        check_in_url.as_str()
    );

    let check_out_url = DriveResource::Me.item_resource("{parent-item-id}", DriveEvent::CheckOut);
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/{parent-item-id}/checkout",
        check_out_url.as_str()
    );
}

#[test]
fn download_format() {
    let url = DriveResource::Drives.item_resource("{item-id}", DriveEvent::DownloadAndFormat);
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drive/items/{item-id}/content?format=",
        url.as_str()
    );

    let pdf = vec![
        DriveResource::Drives.item_resource("{item-id}", DriveEvent::DownloadAndFormat),
        DownloadFormat::PDF.as_ref().into(),
    ];
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drive/items/{item-id}/content?format=pdf",
        pdf.join("")
    );

    let jpg = vec![
        DriveResource::Drives.item_resource("{item-id}", DriveEvent::DownloadAndFormat),
        DownloadFormat::JPG.as_ref().into(),
    ];
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drive/items/{item-id}/content?format=jpg",
        jpg.join("")
    );

    let html = vec![
        DriveResource::Drives.item_resource("{item-id}", DriveEvent::DownloadAndFormat),
        DownloadFormat::HTML.as_ref().into(),
    ];
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drive/items/{item-id}/content?format=html",
        html.join("")
    );

    let glb = vec![
        DriveResource::Drives.item_resource("{item-id}", DriveEvent::DownloadAndFormat),
        DownloadFormat::GLB.as_ref().into(),
    ];
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drive/items/{item-id}/content?format=glb",
        glb.join("")
    );
}
