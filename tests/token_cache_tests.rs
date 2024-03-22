use graph_core::cache::TokenCache;
use std::thread;
use std::time::Duration;
use test_tools::oauth_request::OAuthTestClient;

#[test]
fn token_cache_clone() {
    if let Some(mut credential) = OAuthTestClient::client_secret_credential_default() {
        let token = credential.get_token_silent().unwrap();
        thread::sleep(Duration::from_secs(5));
        let mut credential2 = credential.clone();
        let token2 = credential2.get_token_silent().unwrap();
        assert_eq!(token, token2);
    }
}

#[tokio::test]
async fn token_cache_clone_async() {
    if let Some(mut credential) = OAuthTestClient::client_secret_credential_default() {
        let token = credential.get_token_silent_async().await.unwrap();
        tokio::time::sleep(Duration::from_secs(5)).await;
        let mut credential2 = credential.clone();
        let token2 = credential2.get_token_silent_async().await.unwrap();
        assert_eq!(token, token2);
    }
}
