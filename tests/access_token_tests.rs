use rust_onedrive::flow::accesstoken::AccessToken;

#[test]
fn get_method() {
    let mut access_token =
        AccessToken::new("bearer", 3600, "offline", "ASODFIUJ34KJ;LADSK", None, None);
    assert_eq!(access_token.get_expires_in(), 3600);
    assert_eq!(access_token.get_token_type(), "bearer");
    assert_eq!(
        access_token.get_access_token(),
        "ASODFIUJ34KJ;LADSK"
    );
    assert_eq!(access_token.get_scopes(), "offline");

    access_token.set_refresh_token("eyJh...9323");
    assert_eq!(access_token.get_refresh_token().unwrap(), "eyJh...9323");
}