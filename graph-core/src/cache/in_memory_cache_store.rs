use crate::cache::cache_store::CacheStore;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

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
}

impl<Value: Clone> CacheStore<Value> for InMemoryCacheStore<Value> {
    fn store<T: Into<String>>(&mut self, cache_id: T, token: Value) {
        let mut write_lock = self.store.write();
        write_lock.insert(cache_id.into(), token);
        drop(write_lock);
    }

    fn get(&self, cache_id: &str) -> Option<Value> {
        let read_lock = self.store.read();
        let token = read_lock.get(cache_id).cloned();
        drop(read_lock);
        token
    }

    fn evict(&self, cache_id: &str) -> Option<Value> {
        let mut write_lock = self.store.write();
        let token = write_lock.remove(cache_id);
        drop(write_lock);
        token
    }
}
