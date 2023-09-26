use graph_rs_sdk::oauth::MsalToken;
use std::thread;
use std::time::Duration;

pub fn is_access_token_expired() {
    let mut access_token = MsalToken::default();
    access_token.with_expires_in(1);
    thread::sleep(Duration::from_secs(3));
    assert!(access_token.is_expired());

    let mut access_token = MsalToken::default();
    access_token.with_expires_in(10);
    thread::sleep(Duration::from_secs(4));
    assert!(!access_token.is_expired());
}
