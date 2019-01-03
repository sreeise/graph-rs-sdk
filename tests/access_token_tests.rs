use rust_onedrive::flow::accesstoken::AccessToken;

#[test]
fn get_method() {
    let mut access_token =
        AccessToken::new("bearer", 3600, "offline", "ASODFIUJ34KJ;LADSK", "USER_ID");
    assert_eq!(access_token.get_expires_in().unwrap(), &3600_u64);
    assert_eq!(access_token.get_token_type().unwrap(), "bearer");
    assert_eq!(
        access_token.get_access_token().unwrap(),
        "ASODFIUJ34KJ;LADSK"
    );
    assert_eq!(access_token.get_scopes().unwrap(), "offline");
    assert_eq!(access_token.get_user_id().unwrap(), "USER_ID");

    access_token.set_refresh_token("eyJh...9323");
    assert_eq!(access_token.get_refresh_token().unwrap(), "eyJh...9323");
}

#[test]
fn new_with_refresh_method() {
    let access_token = AccessToken::new_with_refresh_token(
        "bearer",
        3600,
        "offline",
        "ASODFIUJ34KJ;LADSK",
        "USER_ID",
        Some(String::from("eyJh...9323")),
    );
    assert_eq!(access_token.get_expires_in().unwrap(), &3600_u64);
    assert_eq!(access_token.get_token_type().unwrap(), "bearer");
    assert_eq!(
        access_token.get_access_token().unwrap(),
        "ASODFIUJ34KJ;LADSK"
    );
    assert_eq!(access_token.get_scopes().unwrap(), "offline");
    assert_eq!(access_token.get_user_id().unwrap(), "USER_ID");
    assert_eq!(access_token.get_refresh_token().unwrap(), "eyJh...9323");
}
