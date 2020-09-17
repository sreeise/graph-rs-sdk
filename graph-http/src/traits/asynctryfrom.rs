use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait AsyncTryFrom<T, RHS = Self> {
    type Error: Error;

    async fn async_try_from(from: T) -> Result<RHS, Self::Error>;
}
