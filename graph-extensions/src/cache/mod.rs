mod in_memory_credential_store;
mod token_store;
mod token_store_providers;

pub use in_memory_credential_store::*;
use std::fmt::{Debug, Formatter};
pub use token_store::*;
pub use token_store_providers::*;

#[derive(Clone)]
pub struct UnInitializedTokenStore;

impl TokenStore for UnInitializedTokenStore {
    fn token_store_provider(&self) -> TokenStoreProvider {
        TokenStoreProvider::UnInitialized
    }

    fn is_stored_token_initialized(&self, _id: &str) -> bool {
        false
    }

    fn get_stored_token(&self, _id: &str) -> Option<&StoredToken> {
        panic!("UnInitializedTokenStore does not store tokens")
    }

    fn update_stored_token(&mut self, _id: &str, stored_token: StoredToken) -> Option<StoredToken> {
        panic!("UnInitializedTokenStore does not store tokens")
    }

    fn get_bearer_token_from_store(&self, _id: &str) -> Option<&String> {
        info!("Using uninitialized token store - empty string returned for bearer token");
        Default::default()
    }

    fn get_refresh_token_from_store(&self, _id: &str) -> Option<&String> {
        info!("Using uninitialized token store - None returned for refresh token");
        Default::default()
    }
}

impl Debug for UnInitializedTokenStore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("UnInitializedTokenStore")
    }
}
