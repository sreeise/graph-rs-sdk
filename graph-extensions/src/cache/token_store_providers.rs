#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TokenStoreProvider {
    UnInitialized,
    InMemory,
}
