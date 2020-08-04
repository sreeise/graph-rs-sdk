use graph_rs::prelude::*;
use test_tools::oauthrequest::OAuthRequest;
use test_tools::oauthrequest::ASYNC_THROTTLE_MUTEX;

#[tokio::test]
async fn delta_req() {
    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((id, token)) = OAuthRequest::request_access_token_async().await {
        let client = Graph::new_async(token.bearer_token());
        let mut delta_recv = client.v1().users(id.as_str()).delta().send().await;

        let mut is_done = false;

        loop {
            match delta_recv.recv().await {
                Some(delta) => match delta {
                    Delta::Next(response) => {
                        assert!(!is_done);
                        assert!(
                            response.status() == 200 ||
                                response.status() == 201 ||
                                response.status() == 204
                        );
                    },
                    Delta::Done(err) => {
                        if let Some(err) = err {
                            panic!("Request Error. Method: Users delta. Error: {:#?}", err);
                        }
                        is_done = true;
                        break;
                    },
                },
                None => assert!(is_done),
            }
        }
        assert!(is_done);
    }
}
