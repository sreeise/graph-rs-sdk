use serde_derive;
use serde_json;
use std::io;
use std::num::ParseIntError;
use std::path::Path;

/// AccessToken is the token used to make authenticated requests
/// to the OneDrive api
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
/// The refresh token is optional and depends upon whether it was requested.
/// Currently this is not available to be set for the AccessToken.
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AccessToken {
    token_type: String,
    expires_in: u64,
    scope: String,
    access_token: String,
    user_id: String,
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

    pub fn from_json_str(json_str: &str) -> io::Result<AccessToken> {
        let access_token: AccessToken = serde_json::from_str(json_str)?;
        Ok(access_token)
    }
}
