#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TokenCacheProviderType {
    UnInitialized,
    InMemory,
    Session,
    Distributed,
}
