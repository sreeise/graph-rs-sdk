use graph_rs_sdk::oauth::Token;
use std::thread;
use std::time::Duration;

pub fn is_access_token_expired() {
    let mut token = Token::default();
    token.with_expires_in(1);
    thread::sleep(Duration::from_secs(3));
    assert!(token.is_expired());

    let mut token = Token::default();
    token.with_expires_in(10);
    thread::sleep(Duration::from_secs(4));
    assert!(!token.is_expired());
}
