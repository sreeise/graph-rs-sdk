use test_tools::oauthrequest::THROTTLE_MUTEX;
use test_tools::oauthrequest::{Environment, OAuthTestClient};

#[test]
fn user_request_test() {
    if Environment::is_appveyor() {
        return;
    }

    let _lock = THROTTLE_MUTEX.lock().unwrap();
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let users = client.v1().users().list_user().send();

        if let Ok(response) = users {
            assert!(
                response.status() == 200 || response.status() == 201 || response.status() == 204
            );
            let value = response.body().odata_context().cloned().unwrap();
            assert_eq!(
                "https://graph.microsoft.com/v1.0/$metadata#users",
                value.as_str()
            );
        } else if let Err(e) = users {
            panic!("Request error. Method: users list. Error: {:#?}", e);
        }

        let user_res = client.v1().users().id(id).get_user().send();

        if let Ok(response) = user_res {
            assert!(
                response.status() == 200 || response.status() == 201 || response.status() == 204
            );
        } else if let Err(e) = user_res {
            panic!("Request error. Method: users list. Error: {:#?}", e);
        }
    }
}
