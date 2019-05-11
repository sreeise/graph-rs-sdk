use crate::oautherror::OAuthError;
use serde::{Deserialize, Serialize};
use transform_request::prelude::*;

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, FromFile, ToFile)]
pub struct IdToken {
    code: Option<String>,
    id_token: String,
    state: Option<String>,
    session_state: Option<String>,
}

impl IdToken {
    pub fn new(id_token: &str) -> IdToken {
        IdToken {
            code: None,
            id_token: id_token.into(),
            state: None,
            session_state: None,
        }
    }

    pub fn id_token(&mut self, id_token: &str) {
        self.id_token = id_token.into();
    }

    pub fn code(&mut self, code: &str) {
        self.code = Some(code.into());
    }

    pub fn state(&mut self, state: &str) {
        self.state = Some(state.into());
    }

    pub fn session_state(&mut self, session_state: &str) {
        self.session_state = Some(session_state.into());
    }

    pub fn get_id_token(&self) -> String {
        self.id_token.clone()
    }

    pub fn get_code(&self) -> Option<String> {
        self.code.clone()
    }

    pub fn get_state(&self) -> Option<String> {
        self.state.clone()
    }

    pub fn get_session_state(&self) -> Option<String> {
        self.session_state.clone()
    }
}

impl Transform<String> for IdToken {
    type Err = OAuthError;

    fn transform(rhs: String) -> Result<Self, Self::Err>
    where
        Self: Serialize + for<'de> Deserialize<'de>,
    {
        let id_token: IdToken = serde_json::from_str(rhs.as_str())?;
        Ok(id_token)
    }
}
