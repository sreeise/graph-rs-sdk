pub trait CacheStore<Value> {
    /// Store Value given cache id.
    fn store<T: Into<String>>(&mut self, cache_id: T, token: Value);

    /// Get Value from cache given matching cache id.
    fn get(&self, cache_id: &str) -> Option<Value>;

    /// Evict or remove value from cache given cache id.
    fn evict(&self, cache_id: &str) -> Option<Value>;
}
