mod in_memory_credential_store;
mod token_cache_providers;

pub use in_memory_credential_store::*;
pub use token_cache_providers::*;

use crate::oauth::AccessToken;

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum CredentialStoreType {
    Bearer(String),
    AccessToken(AccessToken),
    UnInitialized,
}

pub trait CredentialStore {
    fn token_cache_provider(&self) -> TokenCacheProviderType {
        TokenCacheProviderType::UnInitialized
    }

    fn get_token(&self) -> &CredentialStoreType {
        &CredentialStoreType::UnInitialized
    }

    fn update(&mut self, _credential_store_type: CredentialStoreType) {}

    fn get_token_by_client_id(&self, client_id: &str) -> &CredentialStoreType;

    fn update_by_client_id(
        &mut self,
        _client_id: &str,
        _credential_store_type: CredentialStoreType,
    ) {
    }
}

pub(crate) struct UnInitializedCredentialStore;

impl CredentialStore for UnInitializedCredentialStore {
    fn get_token_by_client_id(&self, _client_id: &str) -> &CredentialStoreType {
        info!("UnInitializedCredentialStore");
        &CredentialStoreType::UnInitialized
    }
}
