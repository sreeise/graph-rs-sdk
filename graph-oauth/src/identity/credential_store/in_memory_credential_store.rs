use crate::access_token::AccessToken;
use crate::identity::{CredentialStore, CredentialStoreType, TokenCacheProviderType};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct InMemoryCredentialStore {
    credentials: BTreeMap<String, CredentialStoreType>,
}

impl InMemoryCredentialStore {
    pub fn new() -> InMemoryCredentialStore {
        InMemoryCredentialStore {
            credentials: BTreeMap::new(),
        }
    }

    pub fn from_bearer_token<T: AsRef<str>>(
        client_id: T,
        bearer_token: T,
    ) -> InMemoryCredentialStore {
        let mut credentials = BTreeMap::new();
        credentials.insert(
            client_id.as_ref().to_owned(),
            CredentialStoreType::Bearer(bearer_token.as_ref().to_owned()),
        );
        InMemoryCredentialStore { credentials }
    }

    pub fn from_access_token<T: AsRef<str>>(
        client_id: T,
        access_token: AccessToken,
    ) -> InMemoryCredentialStore {
        let mut credentials = BTreeMap::new();
        credentials.insert(
            client_id.as_ref().to_owned(),
            CredentialStoreType::AccessToken(access_token),
        );
        InMemoryCredentialStore { credentials }
    }
}

impl CredentialStore for InMemoryCredentialStore {
    fn token_cache_provider(&self) -> TokenCacheProviderType {
        TokenCacheProviderType::InMemory
    }

    fn get_token_by_client_id(&self, client_id: &str) -> &CredentialStoreType {
        info!("InMemoryCredentialStore");
        self.credentials
            .get(client_id)
            .unwrap_or_else(|| &CredentialStoreType::UnInitialized)
    }

    fn update_by_client_id(&mut self, client_id: &str, credential_store_type: CredentialStoreType) {
        info!("InMemoryCredentialStore");
        self.credentials
            .insert(client_id.to_owned(), credential_store_type);
    }
}
