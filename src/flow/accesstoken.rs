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
///     AccessToken::new("bearer", 3600, "offline", "ASODFIUJ34KJ;LADSK", None, Some("USER_ID".to_string()), None);
///
///        assert_eq!(access_token.get_expires_in(), 3600);
///        assert_eq!(access_token.get_token_type(), "bearer");
///        assert_eq!(access_token.get_access_token(), "ASODFIUJ34KJ;LADSK");
///        assert_eq!(access_token.get_scopes(), "offline");
///        assert_eq!(access_token.get_user_id().unwrap(), "USER_ID");
///```
/// The refresh token is optional and depends upon whether it was requested.
/// Currently this is not available to be set for the AccessToken.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccessToken {
    token_type: String,
    expires_in: i64,
    scope: String,
    access_token: String,
    refresh_token: Option<String>,
    user_id: Option<String>,
    id_token: Option<String>,
}

impl AccessToken {
    pub fn new(
        token_type: &str,
        expires_in: i64,
        scope: &str,
        access_token: &str,
        refresh_token: Option<String>,
        user_id: Option<String>,
        id_token: Option<String>, // Azure AD Id Token
    ) -> AccessToken {
        AccessToken {
            token_type: token_type.to_string(),
            expires_in,
            scope: scope.to_string(),
            access_token: access_token.to_string(),
            user_id,
            refresh_token,
            id_token,
        }
    }

    pub fn get_expires_in(&self) -> i64 {
        self.expires_in.clone()
    }

    pub fn get_scopes(&self) -> String {
        self.scope.clone()
    }

    pub fn get_access_token(&self) -> String {
        self.access_token.clone()
    }

    pub fn get_user_id(&self) -> Option<String> {
        self.user_id.clone()
    }

    pub fn get_token_type(&self) -> String {
        self.token_type.clone()
    }

    pub fn get_refresh_token(self) -> Option<String> {
        self.refresh_token.clone()
    }

    pub fn set_refresh_token(&mut self, refresh_token: &str) {
        self.refresh_token = Some(refresh_token.into());
    }

    pub fn set_token_type(&mut self, token_type: &str) {
        self.token_type = token_type.to_string();
    }

    pub fn set_expires_in(&mut self, expires_in: i64) {
        self.expires_in = expires_in;
    }

    pub fn set_scope(&mut self, scope: &str) {
        self.scope = scope.to_string();
    }

    pub fn set_access_token(&mut self, access_token: &str) {
        self.access_token = access_token.to_string();
    }

    pub fn set_user_id(&mut self, user_id: &str) {
        self.user_id = Some(user_id.to_string());
    }

    pub fn set_id_token(&mut self, id_token: &str) {
        self.id_token = Some(id_token.to_string());
    }

    pub fn from_json_str(json_str: &str) -> io::Result<AccessToken> {
        let access_token: AccessToken = serde_json::from_str(json_str)?;
        Ok(access_token)
    }

    pub fn is_expired(&mut self) -> bool {
        unimplemented!();
    }
}
