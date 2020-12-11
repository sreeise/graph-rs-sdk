use std::thread;
use std::time::Duration;
use test_tools::oauthrequest::THROTTLE_MUTEX;
use test_tools::oauthrequest::{Environment, OAuthTestClient};

#[test]
fn get_drafts_mail_folder() {
    if Environment::is_appveyor() {
        return;
    }

    let _lock = THROTTLE_MUTEX.lock().unwrap();

    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let result = client.v1()
            .user(id.as_str())
            .mail_folder("drafts")
            .get_mail_folders()
            .send();

        if let Ok(response) = result {
            let display_name = response.body()["displayName"].as_str().unwrap();
            assert_eq!("Drafts", display_name);
        } else if let Err(e) = result {
            println!("{:#?}", e);
            panic!("Request error. Method: mail_folder get_mail_folder drafts. Error:\n{:#?}", e);
        }
    }
}
