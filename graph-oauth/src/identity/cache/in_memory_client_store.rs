use std::sync::{Arc, RwLock};

use crate::oauth::TokenCredentialExecutor;

#[derive(Clone)]
pub struct InMemoryClientStore<Client: TokenCredentialExecutor, Token> {
    client: Box<Client>,
    token: Arc<RwLock<Token>>,
}
