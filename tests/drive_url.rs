use graph_rs_sdk::prelude::*;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
pub fn drive_me() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/drive".to_string(),
        client.me().default_drive().get_drive().url().path()
    );
    assert_eq!(
        "/v1.0/me/drive/root".to_string(),
        client.me().default_drive().get_root().url().path()
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
        client.site(RID).default_drive().get_drive().url().path()
    );
    assert_eq!(
        "/v1.0/sites/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root".to_string(),
        client.site(RID).default_drive().get_root().url().path()
    );
}

#[test]
fn users() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/users/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive".to_string(),
        client.user(RID).default_drive().get_drive().url().path()
    );
    assert_eq!(
        "/v1.0/users/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root".to_string(),
        client.user(RID).default_drive().get_root().url().path()
    );
}

#[test]
pub fn drive_preview_path() {
    let client = Graph::new("");

    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/drive/root:/Documents/preview.txt:/preview".to_string(),
        client
            .me()
            .default_drive()
            .item_by_path(":/Documents/preview.txt:")
            .preview(&serde_json::json!({}))
            .url()
            .path()
    );
    assert_eq!("/v1.0/users/T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x/drive/root:/Documents/preview.txt:/preview".to_string(), client.user(RID).default_drive().item_by_path(":/Documents/preview.txt:").preview(&serde_json::json!({})).url().path());
}
