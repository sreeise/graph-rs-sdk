use crate::cache::AsBearer;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone, Default)]
pub struct InMemoryCredentialStore<Token: AsBearer + Clone> {
    store: Arc<RwLock<HashMap<String, Token>>>,
}

impl<Token: AsBearer + Clone> InMemoryCredentialStore<Token> {
    pub fn new() -> InMemoryCredentialStore<Token> {
        InMemoryCredentialStore {
            store: Default::default(),
        }
    }

    pub fn store<T: Into<String>>(&mut self, cache_id: T, token: Token) {
        let mut store = self.store.write().unwrap();
        store.insert(cache_id.into(), token);
    }

    pub fn get(&self, cache_id: &str) -> Option<Token> {
        let store = self.store.read().unwrap();
        store.get(cache_id).cloned()
    }
}
