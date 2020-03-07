use graph_rs::prelude::*;
use test_tools::oauthrequest::*;

#[test]
fn delta_req() {
    let _lock = THROTTLE_MUTEX.lock().unwrap();
    OAuthRequest::access_token_fn(|t| {
        if let Some((id, bearer)) = t {
            let client = Graph::from(bearer);
            let delta_recv = client.v1().users(id.as_str()).delta().value();
            let mut is_done = false;

            loop {
                match delta_recv.recv() {
                    Ok(delta) => match delta {
                        Delta::Next(response) => {
                            assert!(!is_done);
                            assert!(response.error().is_none());
                        },
                        Delta::Done(err) => {
                            if let Some(err) = err {
                                panic!(
                                    "Request Error. Method: Users delta. Error: {:#?}",
                                    err
                                );
                            }
                            is_done = true;
                            break;
                        },
                    },
                    Err(err) => {
                        panic!(
                            "Request Error. Method: Users delta. Error: {:#?}",
                            err
                        );
                    },
                }
            }
            assert!(is_done);
        }
    });
}
