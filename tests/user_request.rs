use graph_rs::prelude::*;
use std::error::Error;
use test_tools::oauthrequest::OAuthRequest;

#[test]
fn user_request_test() {
    OAuthRequest::test_credentials(|t| {
        if let Some((id, bearer)) = t {
            let client = Graph::new(bearer.as_str());
            let users = client.v1().users(id.as_str()).list().value();

            if let Ok(response) = users {
                assert!(response.error().is_none());
            } else if let Err(e) = users {
                panic!(
                    "Request error. Method: users list. Error: {:#?}",
                    e.description()
                );
            }

            let user_res = client.v1().users(id.as_str()).get().value();

            if let Ok(response) = user_res {
                assert!(response.error().is_none());
            } else if let Err(e) = user_res {
                panic!(
                    "Request error. Method: users list. Error: {:#?}",
                    e.description()
                );
            }
        }
    })
}
