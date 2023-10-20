use crate::cache::AsBearer;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone, Default)]
pub struct InMemoryTokenStore<Token: AsBearer + Clone> {
    store: Arc<RwLock<HashMap<String, Token>>>,
}

impl<Token: AsBearer + Clone> InMemoryTokenStore<Token> {
    pub fn new() -> InMemoryTokenStore<Token> {
        InMemoryTokenStore {
            store: Default::default(),
        }
    }

    pub fn store<T: Into<String>>(&mut self, cache_id: T, token: Token) {
        let mut write_lock = self.store.write().unwrap();
        write_lock.insert(cache_id.into(), token);
        drop(write_lock);
    }

    pub fn get(&self, cache_id: &str) -> Option<Token> {
        let read_lock = self.store.read().unwrap();
        let token = read_lock.get(cache_id).cloned();
        drop(read_lock);
        token
    }
}
