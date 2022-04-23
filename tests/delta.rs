use graph_core::resource::ResourceIdentity;
use graph_rs_sdk::prelude::*;
use test_tools::oauthrequest::*;

#[test]
fn delta_req() {
    let _lock = THROTTLE_MUTEX.lock().unwrap();
    if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let delta_recv = client.v1().users().delta().send();
        let mut is_done = false;

        loop {
            match delta_recv.recv() {
                Ok(delta) => match delta {
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
                Err(err) => {
                    panic!("Request Error. Method: Users delta. Error: {:#?}", err);
                }
            }
        }
        assert!(is_done);
    }
}

#[test]
fn delta_token_req() {
    let _lock = THROTTLE_MUTEX.lock().unwrap();
    if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let delta_recv = client.v1().users().delta_token("latest").send();
        let mut is_done = false;

        loop {
            match delta_recv.recv() {
                Ok(delta) => match delta {
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
                Err(err) => {
                    panic!("Request Error. Method: Users delta. Error: {:#?}", err);
                }
            }
        }
        assert!(is_done);
    }
}

#[test]
fn delta_token_query() {
    if !Environment::is_local() {
        return;
    }

    let _lock = THROTTLE_MUTEX.lock().unwrap();
    if let Some((_id, client)) = OAuthTestClient::graph_by_rid(ResourceIdentity::Applications) {
        let delta_recv = client.v1().applications().delta().delta_token("latest").send();
        let mut is_done = false;

        loop {
            match delta_recv.recv() {
                Ok(delta) => match delta {
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
                Err(err) => {
                    panic!("Request Error. Method: Users delta. Error: {:#?}", err);
                }
            }
        }
        assert!(is_done);
    }
}
