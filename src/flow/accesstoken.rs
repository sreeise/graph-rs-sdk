use serde_json;
use std::io;

/// AccessToken is the token used to make authenticated requests
/// to the OneDrive api
///
/// Unless the caller is implementing their own version of an AccessToken request,
/// using the new method to create an AccessToken should most likely never happen.
/// Instead, callers should use AuthFlow request_access_token().
/// AuthFlow has a field for the AccessToken struct and is automatically set
/// when calling request_access_token().
///
/// An access token request returns the following:
///
/// # Example
/// ```rust,ignore
/// {
///  "token_type":"bearer",
///  "expires_in": 3600,
///  "scope":"wl.basic onedrive.readwrite",
///  "access_token":"EwCo...AA==",
///  "refresh_token":"eyJh...9323"
///  "user_id" "AAAAA"
///}
/// ```
///
/// An access token can be created by providing the expires in time, token type,
/// access token, scopes, and user id:
///
/// # Example
/// ```
///  use rust_onedrive::flow::accesstoken::AccessToken;
///
///  let access_token =
///     AccessToken::new("bearer", 3600, "offline", "ASODFIUJ34KJ;LADSK", "USER_ID");
///
///        assert_eq!(access_token.get_expires_in().unwrap(), &3600_u64);
///        assert_eq!(access_token.get_token_type().unwrap(), "bearer");
///        assert_eq!(access_token.get_access_token().unwrap(), "ASODFIUJ34KJ;LADSK");
///        assert_eq!(access_token.get_scopes().unwrap(), "offline");
///        assert_eq!(access_token.get_user_id().unwrap(), "USER_ID");
///```
/// The refresh token is optional and depends upon whether it was requested.
/// Currently this is not available to be set for the AccessToken.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccessToken {
    token_type: String,
    expires_in: u64,
    scope: String,
    access_token: String,
    user_id: String,
    refresh_token: Option<String>,
}

impl AccessToken {
    pub fn new(
        token_type: &str,
        expires_in: u64,
        scope: &str,
        access_token: &str,
        user_id: &str,
    ) -> AccessToken {
        AccessToken {
            token_type: String::from(token_type),
            expires_in,
            scope: String::from(scope),
            access_token: String::from(access_token),
            user_id: String::from(user_id),
            refresh_token: None,
        }
    }

    pub fn new_with_refresh_token(
        token_type: &str,
        expires_in: u64,
        scope: &str,
        access_token: &str,
        user_id: &str,
        refresh_token: Option<String>,
    ) -> AccessToken {
        AccessToken {
            token_type: String::from(token_type),
            expires_in,
            scope: String::from(scope),
            access_token: String::from(access_token),
            user_id: String::from(user_id),
            refresh_token,
        }
    }

    pub fn get_expires_in(&self) -> io::Result<&u64> {
        Ok(&self.expires_in)
    }

    pub fn get_scopes(&self) -> io::Result<&str> {
        Ok(&self.scope)
    }

    pub fn get_access_token(&self) -> io::Result<&str> {
        Ok(&self.access_token)
    }

    pub fn get_user_id(&self) -> io::Result<&str> {
        Ok(&self.user_id)
    }

    pub fn get_token_type(&self) -> io::Result<&str> {
        Ok(&self.token_type)
    }

    pub fn get_refresh_token(self) -> Option<String> {
        self.refresh_token
    }

    pub fn set_refresh_token(&mut self, refresh_token: &str) {
        self.refresh_token = Some(String::from(refresh_token));
    }

    pub fn from_json_str(json_str: &str) -> io::Result<AccessToken> {
        let access_token: AccessToken = serde_json::from_str(json_str)?;
        Ok(access_token)
    }
}

#[cfg(test)]
mod access_token_tests {
    use super::*;

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
}
