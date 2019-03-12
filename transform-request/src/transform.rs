use serde::{Deserialize, Serialize};
use std::error::Error;

/// Trait for performing transformations of one type to another
/// wrapped in a Result<Self, Self::Err> where Self is the new type
/// and Self::Err is a type that implements trait std::error::Error.
///
/// Transform's purpose is to efficiently return the new type wrapped in a result
/// for types that implement Serde's Serialize and Deserialize.
pub trait Transform<RHS = Self> {
    type Err: Error;

    fn transform(rhs: RHS) -> Result<Self, Self::Err>
    where
        Self: Serialize + for<'de> Deserialize<'de>;
}
