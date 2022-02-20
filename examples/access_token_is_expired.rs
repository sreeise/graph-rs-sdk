use graph_rs_sdk::oauth::AccessToken;
use std::thread;
use std::time::Duration;

fn main() {
    let mut access_token = AccessToken::default();
    access_token.set_expires_in(1);
    thread::sleep(Duration::from_secs(3));
    assert!(access_token.is_expired());

    let mut access_token = AccessToken::default();
    access_token.set_expires_in(10);
    thread::sleep(Duration::from_secs(4));
    assert!(!access_token.is_expired());
}
