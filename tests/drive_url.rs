use graph_rs_sdk::*;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";

#[test]
pub fn drive_me() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/drive".to_string(),
        client.me().drive().get_drive().url().path()
    );
    assert_eq!(
        "/v1.0/me/drive/root".to_string(),
        client.me().drive().get_root().url().path()
    );
}

#[test]
fn drives() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x".to_string(),
        client.drive(RID).get_drive().url().path()
    );
    assert_eq!(
        "/v1.0/drives/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/root".to_string(),
        client.drive(RID).get_root().url().path()
    );
}

#[test]
fn sites() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive".to_string(),
        client.site(RID).drive().get_drive().url().path()
    );
    assert_eq!(
        "/v1.0/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root".to_string(),
        client.site(RID).drive().get_root().url().path()
    );
}

#[test]
fn users() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/users/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive".to_string(),
        client.user(RID).drive().get_drive().url().path()
    );
    assert_eq!(
        "/v1.0/users/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root".to_string(),
        client.user(RID).drive().get_root().url().path()
    );
}

#[test]
pub fn drive_preview_path() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/drive/root:/Documents/preview.txt:/preview".to_string(),
        client
            .me()
            .drive()
            .item_by_path(":/Documents/preview.txt:")
            .preview(&serde_json::json!({}))
            .url()
            .path()
    );
    assert_eq!("/v1.0/users/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents/preview.txt:/preview".to_string(), client.user(RID).drive().item_by_path(":/Documents/preview.txt:").preview(&serde_json::json!({})).url().path());
}

#[test]
pub fn drives_list_items_last_modified_by_user_path() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/drives/drive-id/list/items/listItem-id/lastModifiedByUser".to_string(),
        client
            .drive("drive-id")
            .lists()
            .item("listItem-id")
            .last_modified_by_user()
            .get_last_modified_by_user()
            .url()
            .path()
    );

    assert_eq!(
        "/v1.0/drives/drive-id/items/driveItem-id/lastModifiedByUser".to_string(),
        client
            .drive("drive-id")
            .item("driveItem-id")
            .last_modified_by_user()
            .get_last_modified_by_user()
            .url()
            .path()
    );
}

#[test]
pub fn drives_list_items_created_by_user_path() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/drives/drive-id/list/items/listItem-id/createdByUser".to_string(),
        client
            .drive("drive-id")
            .lists()
            .item("listItem-id")
            .created_by_user()
            .get_created_by_user()
            .url()
            .path()
    );

    assert_eq!(
        "/v1.0/drives/drive-id/items/driveItem-id/createdByUser".to_string(),
        client
            .drive("drive-id")
            .item("driveItem-id")
            .created_by_user()
            .get_created_by_user()
            .url()
            .path()
    );
}

#[test]
pub fn drives_list_path() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/drives/drive-id/list/contentTypes/contentType-id/columns/columnDefinition-id"
            .to_string(),
        client
            .drive("drive-id")
            .lists()
            .content_type("contentType-id")
            .get_columns("columnDefinition-id")
            .url()
            .path()
    );

    assert_eq!(
        "/v1.0/drives/drive-id/list/columns/$count".to_string(),
        client
            .drive("drive-id")
            .list()
            .get_columns_count()
            .url()
            .path()
    );
}

#[test]
pub fn drives_item_workbook() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/drives/drive-id/items/item-id/worksheets/worksheet-id/charts/chart-id".to_string(),
        client
            .drive("drive-id")
            .item("item-id")
            .worksheet("worksheet-id")
            .chart("chart-id")
            .get_charts()
            .url()
            .path()
    );

    assert_eq!(
        "/v1.0/drives/drive-id/items/driveItem-id/workbook/worksheets/workbookWorksheet-id/tables/workbookTable-id/rows".to_string(),
        client.drive("drive-id").item("driveItem-id").workbook()
            .worksheet("workbookWorksheet-id")
            .table("workbookTable-id")
            .rows()
            .list_rows()
            .url()
            .path()
    );

    assert_eq!(
        "/v1.0/me/drive/items/driveItem-id/workbook/worksheets/workbookWorksheet-id/tables/workbookTable-id/rows".to_string(),
        client.me()
            .drive()
            .item("driveItem-id")
            .workbook()
            .worksheet("workbookWorksheet-id")
            .table("workbookTable-id")
            .rows()
            .list_rows()
            .url()
            .path()
    );

    assert_eq!(
        "/v1.0/me/drive/worksheets/workbookWorksheet-id/tables/workbookTable-id/rows".to_string(),
        client
            .me()
            .drive()
            .worksheet("workbookWorksheet-id")
            .table("workbookTable-id")
            .rows()
            .list_rows()
            .url()
            .path()
    );
}
