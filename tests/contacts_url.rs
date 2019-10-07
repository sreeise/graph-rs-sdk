use graph_rs::client::Graph;

static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
fn contacts() {
    let client = Graph::new("");

    let _ = client.v1().me().contacts().list();
    client.url_ref(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/me/contacts",
            url.to_string()
        );
    });
    let _ = client.v1().me().contacts().get(ID);
    client.url_ref(|url|  {assert_eq!("https://graph.microsoft.com/v1.0/me/contacts/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI", url.to_string());});
    let _ = client.v1().me().contacts().delta();
    client.url_ref(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/me/contacts/delta",
            url.to_string()
        );
    });
    let _ = client.v1().me().contacts().create(&serde_json::json!({}));
    client.url_ref(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/me/contacts",
            url.to_string()
        );
    });
    let _ = client
        .v1()
        .me()
        .contacts()
        .update(ID, &serde_json::json!({}));
    client.url_ref(|url|  {assert_eq!("https://graph.microsoft.com/v1.0/me/contacts/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI", url.to_string());});
    let _ = client.v1().me().contacts().delete(ID);
    client.url_ref(|url|  {assert_eq!("https://graph.microsoft.com/v1.0/me/contacts/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI", url.to_string());});
}

#[test]
fn contacts_folder() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .contacts()
        .contacts_folder()
        .contacts()
        .list(ID);
    client.url_ref(|url|  {assert_eq!("https://graph.microsoft.com/v1.0/me/contactfolders/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/contacts", url.to_string());});
    let _ = client
        .v1()
        .me()
        .contacts()
        .contacts_folder()
        .list_child_folders(ID);
    client.url_ref(|url|  {assert_eq!("https://graph.microsoft.com/v1.0/me/contactfolders/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI/childFolders", url.to_string());});
    let _ = client.v1().me().contacts().contacts_folder().get(ID);
    client.url_ref(|url|  {assert_eq!("https://graph.microsoft.com/v1.0/me/contactfolders/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI", url.to_string());});
    let _ = client.v1().me().contacts().contacts_folder().delta();
    client.url_ref(|url| {
        assert_eq!(
            "https://graph.microsoft.com/v1.0/me/contactfolders/delta",
            url.to_string()
        );
    });
    let _ = client
        .v1()
        .me()
        .contacts()
        .contacts_folder()
        .contacts()
        .create(ID, &serde_json::json!({}));
    client.url_ref(|url|  {assert_eq!("https://graph.microsoft.com/v1.0/me/contactfolders/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI", url.to_string());});
    let _ = client
        .v1()
        .me()
        .contacts()
        .contacts_folder()
        .update(ID, &serde_json::json!({}));
    client.url_ref(|url|  {assert_eq!("https://graph.microsoft.com/v1.0/me/contactfolders/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI", url.to_string());});
}
