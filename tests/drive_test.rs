use rust_onedrive::drive::endpoint::DriveEndPoint;
use rust_onedrive::drive::Drive;
use rust_onedrive::drive::DriveAction;
use rust_onedrive::drive::DriveResource;
use rust_onedrive::drive::QueryString;

#[test]
fn checkin_test() {
    /*
    POST /drives/{driveId}/items/{itemId}/checkin
    POST /groups/{groupId}/drive/items/{itemId}/checkin
    POST /me/drive/items/{item-id}/checkin
    POST /sites/{siteId}/drive/items/{itemId}/checkin
    POST /users/{userId}/drive/items/{itemId}/checkin
    */
    let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
    let checkin_item_drive = drive.resource_drive_item_url(
        DriveResource::Drives,
        DriveAction::CheckIn,
        "driveId",
        "itemId",
    );
    let checkin_item_group = drive.resource_drive_item_url(
        DriveResource::Groups,
        DriveAction::CheckIn,
        "groupId",
        "itemId",
    );
    let checkin_item_users = drive.resource_drive_item_url(
        DriveResource::Users,
        DriveAction::CheckIn,
        "userId",
        "itemId",
    );
    let checkin_item_sites = drive.resource_drive_item_url(
        DriveResource::Sites,
        DriveAction::CheckIn,
        "siteId",
        "itemId",
    );
    let checkin_item_me = drive.resource_drive_item_url(
        DriveResource::Me,
        DriveAction::CheckIn,
        "meId_not_used",
        "item-Id",
    );

    assert_eq!(
        checkin_item_drive,
        "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId/checkin"
    );
    assert_eq!(
        checkin_item_group,
        "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId/checkin"
    );
    assert_eq!(
        checkin_item_users,
        "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId/checkin"
    );
    assert_eq!(
        checkin_item_sites,
        "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId/checkin"
    );
    assert_eq!(
        checkin_item_me,
        "https://graph.microsoft.com/v1.0/me/drive/items/item-Id/checkin"
    );
}

#[test]
fn checkout_test() {
    /*
    POST /drives/{driveId}/items/{itemId}/checkout
    POST /groups/{groupId}/drive/items/{itemId}/checkout
    POST /me/drive/items/{item-id}/checkout
    POST /sites/{siteId}/drive/items/{itemId}/checkout
    POST /users/{userId}/drive/items/{itemId}/checkout
    */
    let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
    let checkout_item_drive = drive.resource_drive_item_url(
        DriveResource::Drives,
        DriveAction::CheckOut,
        "driveId",
        "itemId",
    );
    let checkout_item_group = drive.resource_drive_item_url(
        DriveResource::Groups,
        DriveAction::CheckOut,
        "groupId",
        "itemId",
    );
    let checkout_item_users = drive.resource_drive_item_url(
        DriveResource::Users,
        DriveAction::CheckOut,
        "userId",
        "itemId",
    );
    let checkout_item_sites = drive.resource_drive_item_url(
        DriveResource::Sites,
        DriveAction::CheckOut,
        "siteId",
        "itemId",
    );
    let checkout_item_me = drive.resource_drive_item_url(
        DriveResource::Me,
        DriveAction::CheckOut,
        "meId_not_used",
        "item-Id",
    );

    assert_eq!(
        checkout_item_drive,
        "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId/checkout"
    );
    assert_eq!(
        checkout_item_group,
        "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId/checkout"
    );
    assert_eq!(
        checkout_item_users,
        "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId/checkout"
    );
    assert_eq!(
        checkout_item_sites,
        "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId/checkout"
    );
    assert_eq!(
        checkout_item_me,
        "https://graph.microsoft.com/v1.0/me/drive/items/item-Id/checkout"
    );
}

#[test]
fn copy_test() {
    /*
    POST /drives/{driveId}/items/{itemId}/copy
    POST /groups/{groupId}/drive/items/{itemId}/copy
    POST /me/drive/items/{item-id}/copy
    POST /sites/{siteId}/drive/items/{itemId}/copy
    POST /users/{userId}/drive/items/{itemId}/copy
    */
    let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
    let copy_item_drive = drive.resource_drive_item_url(
        DriveResource::Drives,
        DriveAction::Copy,
        "driveId",
        "itemId",
    );
    let copy_item_group = drive.resource_drive_item_url(
        DriveResource::Groups,
        DriveAction::Copy,
        "groupId",
        "itemId",
    );
    let copy_item_users =
        drive.resource_drive_item_url(DriveResource::Users, DriveAction::Copy, "userId", "itemId");
    let copy_item_sites =
        drive.resource_drive_item_url(DriveResource::Sites, DriveAction::Copy, "siteId", "itemId");
    let copy_item_me = drive.resource_drive_item_url(
        DriveResource::Me,
        DriveAction::Copy,
        "meId_not_used",
        "item-Id",
    );

    assert_eq!(
        copy_item_drive,
        "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId/copy"
    );
    assert_eq!(
        copy_item_group,
        "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId/copy"
    );
    assert_eq!(
        copy_item_users,
        "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId/copy"
    );
    assert_eq!(
        copy_item_sites,
        "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId/copy"
    );
    assert_eq!(
        copy_item_me,
        "https://graph.microsoft.com/v1.0/me/drive/items/item-Id/copy"
    );
}

#[test]
fn create_folder_test() {
    /*
    POST /drives/{drive-id}/items/{parent-item-id}/children
    POST /groups/{group-id}/drive/items/{parent-item-id}/children
    POST /me/drive/items/{parent-item-id}/children
    POST /sites/{site-id}/drive/items/{parent-item-id}/children
    POST /users/{user-id}/drive/items/{parent-item-id}/children
    */
    let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
    let create_folder_item_drive = drive.resource_drive_item_url(
        DriveResource::Drives,
        DriveAction::CreateFolder,
        "driveId",
        "itemId",
    );
    let create_folder_item_group = drive.resource_drive_item_url(
        DriveResource::Groups,
        DriveAction::CreateFolder,
        "groupId",
        "itemId",
    );
    let create_folder_item_users = drive.resource_drive_item_url(
        DriveResource::Users,
        DriveAction::CreateFolder,
        "userId",
        "itemId",
    );
    let create_folder_item_sites = drive.resource_drive_item_url(
        DriveResource::Sites,
        DriveAction::CreateFolder,
        "siteId",
        "itemId",
    );
    let create_folder_item_me = drive.resource_drive_item_url(
        DriveResource::Me,
        DriveAction::CreateFolder,
        "meId_not_used",
        "item-Id",
    );

    assert_eq!(
        create_folder_item_drive,
        "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId/children"
    );
    assert_eq!(
        create_folder_item_group,
        "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId/children"
    );
    assert_eq!(
        create_folder_item_users,
        "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId/children"
    );
    assert_eq!(
        create_folder_item_sites,
        "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId/children"
    );
    assert_eq!(
        create_folder_item_me,
        "https://graph.microsoft.com/v1.0/me/drive/items/item-Id/children"
    );
}

#[test]
fn delete_item_test() {
    /*
    DELETE /drives/{drive-id}/items/{item-id}
    DELETE /groups/{group-id}/drive/items/{item-id}
    DELETE /me/drive/items/{item-id}
    DELETE /sites/{siteId}/drive/items/{itemId}
    DELETE /users/{userId}/drive/items/{itemId}
    */
    let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
    let delete_item_drive = drive.resource_drive_item_url(
        DriveResource::Drives,
        DriveAction::Delete,
        "driveId",
        "itemId",
    );
    let delete_item_group = drive.resource_drive_item_url(
        DriveResource::Groups,
        DriveAction::Delete,
        "groupId",
        "itemId",
    );
    let delete_item_users = drive.resource_drive_item_url(
        DriveResource::Users,
        DriveAction::Delete,
        "userId",
        "itemId",
    );
    let delete_item_sites = drive.resource_drive_item_url(
        DriveResource::Sites,
        DriveAction::Delete,
        "siteId",
        "itemId",
    );
    let delete_item_me = drive.resource_drive_item_url(
        DriveResource::Me,
        DriveAction::Delete,
        "meId_not_used",
        "item-Id",
    );

    assert_eq!(
        delete_item_drive,
        "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId"
    );
    assert_eq!(
        delete_item_group,
        "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId"
    );
    assert_eq!(
        delete_item_users,
        "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId"
    );
    assert_eq!(
        delete_item_sites,
        "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId"
    );
    assert_eq!(
        delete_item_me,
        "https://graph.microsoft.com/v1.0/me/drive/items/item-Id"
    );
}

#[test]
fn download_item_test() {
    /*
    DELETE /drives/{drive-id}/items/{item-id}
    DELETE /groups/{group-id}/drive/items/{item-id}
    DELETE /me/drive/items/{item-id}
    DELETE /sites/{siteId}/drive/items/{itemId}
    DELETE /users/{userId}/drive/items/{itemId}
    */
    let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
    let download_item_drive = drive.resource_drive_item_url(
        DriveResource::Drives,
        DriveAction::Download,
        "driveId",
        "itemId",
    );
    let download_item_group = drive.resource_drive_item_url(
        DriveResource::Groups,
        DriveAction::Download,
        "groupId",
        "itemId",
    );
    let download_item_users = drive.resource_drive_item_url(
        DriveResource::Users,
        DriveAction::Download,
        "userId",
        "itemId",
    );
    let download_item_sites = drive.resource_drive_item_url(
        DriveResource::Sites,
        DriveAction::Download,
        "siteId",
        "itemId",
    );
    let download_item_me = drive.resource_drive_item_url(
        DriveResource::Me,
        DriveAction::Download,
        "meId_not_used",
        "item-Id",
    );
    let download_item_me_path = drive.resource_drive_item_url(
        DriveResource::Me,
        DriveAction::Download,
        "meId_not_used",
        "item-Id/itemid:",
    );

    assert_eq!(
        download_item_drive,
        "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId/content"
    );
    assert_eq!(
        download_item_group,
        "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId/content"
    );
    assert_eq!(
        download_item_users,
        "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId/content"
    );
    assert_eq!(
        download_item_sites,
        "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId/content"
    );
    assert_eq!(
        download_item_me,
        "https://graph.microsoft.com/v1.0/me/drive/items/item-Id/content"
    );
    assert_eq!(
        download_item_me_path,
        "https://graph.microsoft.com/v1.0/me/drive/root/item-Id/itemid:/content"
    );
}

#[test]
fn get_item_test() {
    /*
    GET /drives/{drive-id}/items/{item-id}
    GET /groups/{group-id}/drive/items/{item-id}
    GET /me/drive/items/{item-id}
    GET /sites/{siteId}/drive/items/{itemId}
    GET /users/{userId}/drive/items/{itemId}
    */
    let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
    let get_item_drive = drive.resource_drive_item_url(
        DriveResource::Drives,
        DriveAction::GetItem,
        "driveId",
        "itemId",
    );
    let get_item_group = drive.resource_drive_item_url(
        DriveResource::Groups,
        DriveAction::GetItem,
        "groupId",
        "itemId",
    );
    let get_item_users = drive.resource_drive_item_url(
        DriveResource::Users,
        DriveAction::GetItem,
        "userId",
        "itemId",
    );
    let get_item_sites = drive.resource_drive_item_url(
        DriveResource::Sites,
        DriveAction::GetItem,
        "siteId",
        "itemId",
    );
    let get_item_me = drive.resource_drive_item_url(
        DriveResource::Me,
        DriveAction::GetItem,
        "meId_not_used",
        "item-Id",
    );

    assert_eq!(
        get_item_drive,
        "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId"
    );
    assert_eq!(
        get_item_group,
        "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId"
    );
    assert_eq!(
        get_item_users,
        "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId"
    );
    assert_eq!(
        get_item_sites,
        "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId"
    );
    assert_eq!(
        get_item_me,
        "https://graph.microsoft.com/v1.0/me/drive/items/item-Id"
    );
}

#[test]
fn get_item_root_test() {
    /*
    GET /drives/{drive-id}/root:/{item-path}
    GET /groups/{group-id}/drive/root:/{item-path}
    GET /me/drive/root:/{item-path}
    GET /sites/{siteId}/drive/root:/{item-path}
    GET /users/{userId}/drive/root:/{item-path}
    */

    let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
    let root_item_drive = drive.resource_drive_item_url(
        DriveResource::Drives,
        DriveAction::GetItemRoot,
        "driveId",
        "itemId",
    );
    let root_item_group = drive.resource_drive_item_url(
        DriveResource::Groups,
        DriveAction::GetItemRoot,
        "groupId",
        "itemId",
    );
    let root_item_users = drive.resource_drive_item_url(
        DriveResource::Users,
        DriveAction::GetItemRoot,
        "userId",
        "itemId",
    );
    let root_item_sites = drive.resource_drive_item_url(
        DriveResource::Sites,
        DriveAction::GetItemRoot,
        "siteId",
        "itemId",
    );
    let root_item_me = drive.resource_drive_item_url(
        DriveResource::Me,
        DriveAction::GetItemRoot,
        "meId_not_used",
        "item-Id",
    );

    assert_eq!(
        root_item_drive,
        "https://graph.microsoft.com/v1.0/drives/driveId/root:/itemId"
    );
    assert_eq!(
        root_item_group,
        "https://graph.microsoft.com/v1.0/groups/groupId/drive/root:/itemId"
    );
    assert_eq!(
        root_item_users,
        "https://graph.microsoft.com/v1.0/users/userId/drive/root:/itemId"
    );
    assert_eq!(
        root_item_sites,
        "https://graph.microsoft.com/v1.0/sites/siteId/drive/root:/itemId"
    );
    assert_eq!(
        root_item_me,
        "https://graph.microsoft.com/v1.0/me/drive/root:/item-Id"
    );
}

#[test]
fn list_children_test() {
    /*
    GET /drives/{drive-id}/items/{item-id}/children
    GET /groups/{group-id}/drive/items/{item-id}/children
    GET /me/drive/items/{item-id}/children
    GET /sites/{site-id}/drive/items/{item-id}/children
    GET /users/{user-id}/drive/items/{item-id}/children
    */
    let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
    let list_children_drive = drive.resource_drive_item_url(
        DriveResource::Drives,
        DriveAction::ListChildren,
        "driveId",
        "itemId",
    );
    let list_children_group = drive.resource_drive_item_url(
        DriveResource::Groups,
        DriveAction::ListChildren,
        "groupId",
        "itemId",
    );
    let list_children_users = drive.resource_drive_item_url(
        DriveResource::Users,
        DriveAction::ListChildren,
        "userId",
        "itemId",
    );
    let list_children_sites = drive.resource_drive_item_url(
        DriveResource::Sites,
        DriveAction::ListChildren,
        "siteId",
        "itemId",
    );
    let list_children_me = drive.resource_drive_item_url(
        DriveResource::Me,
        DriveAction::ListChildren,
        "meId_not_used",
        "item-Id",
    );

    assert_eq!(
        list_children_drive,
        "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId/children"
    );
    assert_eq!(
        list_children_group,
        "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId/children"
    );
    assert_eq!(
        list_children_users,
        "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId/children"
    );
    assert_eq!(
        list_children_sites,
        "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId/children"
    );
    assert_eq!(
        list_children_me,
        "https://graph.microsoft.com/v1.0/me/drive/items/item-Id/children"
    );
}

#[test]
fn move_test() {
    /*
    PATCH /drives/{drive-id}/items/{item-id}
    PATCH /groups/{group-id}/drive/{item-id}
    PATCH /me/drive/items/{item-id}
    PATCH /sites/{site-id}/drive/items/{item-id}
    PATCH /users/{user-id}/drive/items/{item-id}
    */
    let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
    let move_item_drive = drive.resource_drive_item_url(
        DriveResource::Drives,
        DriveAction::Move,
        "driveId",
        "itemId",
    );
    let move_item_group = drive.resource_drive_item_url(
        DriveResource::Groups,
        DriveAction::Move,
        "groupId",
        "itemId",
    );
    let move_item_users =
        drive.resource_drive_item_url(DriveResource::Users, DriveAction::Move, "userId", "itemId");
    let move_item_sites =
        drive.resource_drive_item_url(DriveResource::Sites, DriveAction::Move, "siteId", "itemId");
    let move_item_me = drive.resource_drive_item_url(
        DriveResource::Me,
        DriveAction::Move,
        "meId_not_used",
        "item-Id",
    );

    assert_eq!(
        move_item_drive,
        "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId"
    );
    assert_eq!(
        move_item_group,
        "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId"
    );
    assert_eq!(
        move_item_users,
        "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId"
    );
    assert_eq!(
        move_item_sites,
        "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId"
    );
    assert_eq!(
        move_item_me,
        "https://graph.microsoft.com/v1.0/me/drive/items/item-Id"
    );
}

#[test]
fn upload_test() {
    /*
    PUT /drives/{drive-id}/items/{item-id}/content
    PUT /groups/{group-id}/drive/items/{item-id}/content
    PUT /me/drive/items/{item-id}/content
    PUT /sites/{site-id}/drive/items/{item-id}/content
    PUT /users/{user-id}/drive/items/{item-id}/content
    */
    let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
    let upload_item_drive = drive.resource_drive_item_url(
        DriveResource::Drives,
        DriveAction::Upload,
        "driveId",
        "itemId",
    );
    let upload_item_group = drive.resource_drive_item_url(
        DriveResource::Groups,
        DriveAction::Upload,
        "groupId",
        "itemId",
    );
    let upload_item_users = drive.resource_drive_item_url(
        DriveResource::Users,
        DriveAction::Upload,
        "userId",
        "itemId",
    );
    let upload_item_sites = drive.resource_drive_item_url(
        DriveResource::Sites,
        DriveAction::Upload,
        "siteId",
        "itemId",
    );
    let upload_item_me = drive.resource_drive_item_url(
        DriveResource::Me,
        DriveAction::Upload,
        "meId_not_used",
        "item-Id",
    );

    assert_eq!(
        upload_item_drive,
        "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId/content"
    );
    assert_eq!(
        upload_item_group,
        "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId/content"
    );
    assert_eq!(
        upload_item_users,
        "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId/content"
    );
    assert_eq!(
        upload_item_sites,
        "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId/content"
    );
    assert_eq!(
        upload_item_me,
        "https://graph.microsoft.com/v1.0/me/drive/items/item-Id/content"
    );
}

#[test]
pub fn list_versions_test() {
    /*
    GET /drives/{drive-id}/items/{item-id}/versions
    GET /groups/{group-id}/drive/{item-id}/versions
    GET /me/drive/items/{item-id}/versions
    GET /sites/{site-id}/drive/items/{item-id}/versions
    GET /users/{user-id}/drive/items/{item-id}/versions
    */
    let drive = Drive::new("Ei4rD32VVoFtDE69nI=");
    let versions_item_drive = drive.resource_drive_item_url(
        DriveResource::Drives,
        DriveAction::ListVersions,
        "driveId",
        "itemId",
    );
    let versions_item_group = drive.resource_drive_item_url(
        DriveResource::Groups,
        DriveAction::ListVersions,
        "groupId",
        "itemId",
    );
    let versions_item_users = drive.resource_drive_item_url(
        DriveResource::Users,
        DriveAction::ListVersions,
        "userId",
        "itemId",
    );
    let versions_item_sites = drive.resource_drive_item_url(
        DriveResource::Sites,
        DriveAction::ListVersions,
        "siteId",
        "itemId",
    );
    let versions_item_me = drive.resource_drive_item_url(
        DriveResource::Me,
        DriveAction::ListVersions,
        "meId_not_used",
        "item-Id",
    );

    assert_eq!(
        versions_item_drive,
        "https://graph.microsoft.com/v1.0/drives/driveId/items/itemId/versions"
    );
    assert_eq!(
        versions_item_group,
        "https://graph.microsoft.com/v1.0/groups/groupId/drive/items/itemId/versions"
    );
    assert_eq!(
        versions_item_users,
        "https://graph.microsoft.com/v1.0/users/userId/drive/items/itemId/versions"
    );
    assert_eq!(
        versions_item_sites,
        "https://graph.microsoft.com/v1.0/sites/siteId/drive/items/itemId/versions"
    );
    assert_eq!(
        versions_item_me,
        "https://graph.microsoft.com/v1.0/me/drive/items/item-Id/versions"
    );
}

#[test]
fn query_string_test() {
    let expand_query = vec!["id", "name", "size"];
    let select_query = vec!["name", "size"];
    let drive = Drive::new("access_token");

    let expand_string = drive.expand_url(DriveEndPoint::DriveRoot, "children", &expand_query);
    assert_eq!(
        expand_string,
        "https://graph.microsoft.com/v1.0/drive/root?expand=children(select=id,name,size)"
    );

    let select_string = drive.select_url(DriveEndPoint::DriveRootChild, &select_query);
    assert_eq!(
        select_string,
        "https://graph.microsoft.com/v1.0/drive/root/children?select=name,size"
    );
}
