use crate::cache::TokenStoreProvider;
use crate::token::MsalToken;
use dyn_clone::DynClone;

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum StoredToken {
    BearerToken(String),
    MsalToken(MsalToken),
    BearerAndRefreshToken { bearer: String, refresh: String },
    UnInitialized,
}

impl StoredToken {
    pub fn is_initialized(&self) -> bool {
        !self.eq(&StoredToken::UnInitialized)
    }

    pub fn enable_pii_logging(&mut self) {
        if let StoredToken::MsalToken(token) = self {
            token.enable_pii_logging(true);
        }
    }

    pub fn get_bearer_token(&self) -> Option<&String> {
        match self {
            StoredToken::BearerToken(bearer) => Some(bearer),
            StoredToken::MsalToken(msal_token) => Some(&msal_token.access_token),
            StoredToken::BearerAndRefreshToken { bearer, refresh: _ } => Some(bearer),
            StoredToken::UnInitialized => None,
        }
    }

    pub fn get_refresh_token(&self) -> Option<&String> {
        match self {
            StoredToken::BearerToken(_) => None,
            StoredToken::MsalToken(msal_token) => msal_token.refresh_token.as_ref(),
            StoredToken::BearerAndRefreshToken { bearer: _, refresh } => Some(refresh),
            StoredToken::UnInitialized => None,
        }
    }
}

dyn_clone::clone_trait_object!(TokenStore);

pub trait TokenStore: DynClone {
    fn token_store_provider(&self) -> TokenStoreProvider;

    fn is_token_store_initialized(&self) -> bool {
        !self
            .token_store_provider()
            .eq(&TokenStoreProvider::UnInitialized)
    }

    fn is_stored_token_initialized(&self, id: &str) -> bool;

    fn is_store_and_token_initialized(&self, id: &str) -> bool {
        self.is_token_store_initialized() && self.is_stored_token_initialized(id)
    }

    fn get_stored_token(&self, id: &str) -> Option<&StoredToken>;

    fn update_stored_token(&mut self, id: &str, stored_token: StoredToken) -> Option<StoredToken>;

    fn get_bearer_token_from_store(&self, id: &str) -> Option<&String>;

    fn get_refresh_token_from_store(&self, id: &str) -> Option<&String>;
}
