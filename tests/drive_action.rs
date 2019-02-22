use rust_onedrive::drive::driveaction::DriveAction;
use rust_onedrive::drive::driveresource::DriveResource;

#[test]
fn drive_action_url() {
    let check_in_url = DriveResource::Drives.action_url(
        Some("{drive-id}"),
        "{parent-item-id}",
        DriveAction::CheckIn,
    );
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/{drive-id}/items/{parent-item-id}/checkin",
        check_in_url.as_str()
    );

    let check_out_url = DriveResource::Drives.action_url(
        Some("{drive-id}"),
        "{parent-item-id}",
        DriveAction::CheckOut,
    );
    assert_eq!(
        "https://graph.microsoft.com/v1.0/drives/{drive-id}/items/{parent-item-id}/checkout",
        check_out_url.as_str()
    );
}

#[test]
fn groups_action_url() {
    let check_in_url = DriveResource::Groups.action_url(
        Some("{drive-id}"),
        "{parent-item-id}",
        DriveAction::CheckIn,
    );
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/{drive-id}/drive/items/{parent-item-id}/checkin",
        check_in_url.as_str()
    );

    let check_out_url = DriveResource::Groups.action_url(
        Some("{drive-id}"),
        "{parent-item-id}",
        DriveAction::CheckOut,
    );
    assert_eq!(
        "https://graph.microsoft.com/v1.0/groups/{drive-id}/drive/items/{parent-item-id}/checkout",
        check_out_url.as_str()
    );
}

#[test]
fn sites_action_url() {
    let check_in_url = DriveResource::Sites.action_url(
        Some("{drive-id}"),
        "{parent-item-id}",
        DriveAction::CheckIn,
    );
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/{drive-id}/drive/items/{parent-item-id}/checkin",
        check_in_url.as_str()
    );

    let check_out_url = DriveResource::Sites.action_url(
        Some("{drive-id}"),
        "{parent-item-id}",
        DriveAction::CheckOut,
    );
    assert_eq!(
        "https://graph.microsoft.com/v1.0/sites/{drive-id}/drive/items/{parent-item-id}/checkout",
        check_out_url.as_str()
    );
}

#[test]
fn users_action_url() {
    let check_in_url = DriveResource::Users.action_url(
        Some("{drive-id}"),
        "{parent-item-id}",
        DriveAction::CheckIn,
    );
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/{drive-id}/drive/items/{parent-item-id}/checkin",
        check_in_url.as_str()
    );

    let check_out_url = DriveResource::Users.action_url(
        Some("{drive-id}"),
        "{parent-item-id}",
        DriveAction::CheckOut,
    );
    assert_eq!(
        "https://graph.microsoft.com/v1.0/users/{drive-id}/drive/items/{parent-item-id}/checkout",
        check_out_url.as_str()
    );
}

#[test]
fn me_action_url() {
    let check_in_url = DriveResource::Me.action_url(None, "{parent-item-id}", DriveAction::CheckIn);
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/{parent-item-id}/checkin",
        check_in_url.as_str()
    );

    let check_out_url =
        DriveResource::Me.action_url(None, "{parent-item-id}", DriveAction::CheckOut);
    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive/items/{parent-item-id}/checkout",
        check_out_url.as_str()
    );
}
