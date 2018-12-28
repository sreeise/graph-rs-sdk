use serde_derive;
use serde_json;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
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
    expires_in: String,
    scope: String,
    access_token: String,
    user_id: String,
}

impl AccessToken {
    pub fn new(
        token_type: String,
        expires_in: String,
        scope: String,
        access_token: String,
        user_id: String,
    ) -> AccessToken {
        AccessToken {
            token_type,
            expires_in,
            scope,
            access_token,
            user_id,
        }
    }

    pub fn get_expires_in(&self) -> Option<u64> {
        let num = self.expires_in.parse::<u64>();
        match num {
            Ok(u64_num) => Some(u64_num),
            Err(error) => None,
        }
    }

    pub fn get_scopes(&self) -> &String {
        &self.scope
    }

    pub fn get_access_token(&self) -> &String {
        &self.access_token
    }

    pub fn get_user_id(&self) -> &String {
        &self.user_id
    }

    pub fn get_token_type(&self) -> &String {
        &self.token_type
    }

    pub fn from_json_str(json_str: &str) -> io::Result<AccessToken> {
        let access_token: AccessToken = serde_json::from_str(json_str)?;
        Ok(access_token)
    }
}
