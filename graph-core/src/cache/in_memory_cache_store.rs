use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone, Default)]
pub struct InMemoryCacheStore<Value: Clone> {
    store: Arc<RwLock<HashMap<String, Value>>>,
}

impl<Value: Clone> InMemoryCacheStore<Value> {
    pub fn new() -> InMemoryCacheStore<Value> {
        InMemoryCacheStore {
            store: Default::default(),
        }
    }

    pub fn store<T: Into<String>>(&mut self, cache_id: T, token: Value) {
        let mut write_lock = self.store.write().unwrap();
        write_lock.insert(cache_id.into(), token);
        drop(write_lock);
    }

    pub fn get(&self, cache_id: &str) -> Option<Value> {
        let read_lock = self.store.read().unwrap();
        let token = read_lock.get(cache_id).cloned();
        drop(read_lock);
        token
    }
}
