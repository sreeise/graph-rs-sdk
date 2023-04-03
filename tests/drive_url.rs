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
    let _client = Graph::new("");

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
