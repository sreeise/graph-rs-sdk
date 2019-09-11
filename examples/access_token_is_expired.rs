use graph_rs::oauth::AccessToken;
use std::thread;
use std::time::Duration;

fn main() {
    let mut access_token = AccessToken::default();
    access_token.expires_in(1);
    thread::sleep(Duration::from_secs(3));
    assert_eq!(access_token.is_expired(), true);

    let mut access_token = AccessToken::default();
    access_token.expires_in(10);
    thread::sleep(Duration::from_secs(4));
    assert_eq!(access_token.is_expired(), false);
}
