use graph_rs_sdk::prelude::*;
use test_tools::oauthrequest::ASYNC_THROTTLE_MUTEX;
use test_tools::oauthrequest::{Environment, OAuthTestClient};

#[tokio::test]
async fn delta_req() {
    if Environment::is_travis() || Environment::is_appveyor() {
        return;
    }

    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let mut delta_recv = client.v1().users().delta().send().await;

        let mut is_done = false;

        loop {
            match delta_recv.recv().await {
                Some(delta) => match delta {
                    Delta::Next(response) => {
                        assert!(!is_done);
                        assert!(
                            response.status() == 200
                                || response.status() == 201
                                || response.status() == 204
                        );
                    }
                    Delta::Done(err) => {
                        if let Some(err) = err {
                            panic!("Request Error. Method: Users delta. Error: {:#?}", err);
                        }
                        is_done = true;
                        break;
                    }
                },
                None => assert!(is_done),
            }
        }
        assert!(is_done);
    }
}
