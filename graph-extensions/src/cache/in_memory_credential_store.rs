use crate::cache::{StoredToken, TokenStore, TokenStoreProvider};
use std::collections::HashMap;

#[derive(Clone)]
pub struct InMemoryCredentialStore {
    store: HashMap<String, StoredToken>,
}

impl InMemoryCredentialStore {
    pub fn new(id: String, stored_token: StoredToken) -> InMemoryCredentialStore {
        let mut store = HashMap::new();
        store.insert(id, stored_token);

        InMemoryCredentialStore { store }
    }
}

impl TokenStore for InMemoryCredentialStore {
    fn token_store_provider(&self) -> TokenStoreProvider {
        TokenStoreProvider::InMemory
    }

    fn is_stored_token_initialized(&self, id: &str) -> bool {
        if let Some(stored_token) = self.store.get(id) {
            stored_token.is_initialized()
        } else {
            false
        }
    }

    fn get_stored_token(&self, id: &str) -> Option<&StoredToken> {
        self.store.get(id)
    }

    fn update_stored_token(&mut self, id: &str, stored_token: StoredToken) -> Option<StoredToken> {
        self.store.insert(id.to_string(), stored_token)
    }

    fn get_bearer_token_from_store(&self, id: &str) -> Option<&String> {
        if let Some(stored_token) = self.store.get(id) {
            stored_token.get_bearer_token()
        } else {
            None
        }
    }

    fn get_refresh_token_from_store(&self, id: &str) -> Option<&String> {
        if let Some(stored_token) = self.store.get(id) {
            stored_token.get_refresh_token()
        } else {
            None
        }
    }
}
