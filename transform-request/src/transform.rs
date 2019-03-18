use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;

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

/// Writes a data struct to a file given. The struct must implement
/// serde_derive Serialize and Deserialize.
/// # Arguments
///
/// * `path` - Path to a file and the file name itself.
pub trait ToFile<RHS = Self>
where
    Self: serde::Serialize,
    for<'de> Self: serde::Deserialize<'de>,
{
    type Err: Error;
    type Output;

    fn to_file<P: AsRef<Path>>(&self, path: P) -> std::result::Result<Self::Output, Self::Err>;
}

/// Returns a struct from a given file. The struct must implement
/// serde_derive Serialize and Deserialize.
/// # Arguments
///
/// * `path` - Path to a file and the file name itself.
pub trait FromFile<RHS = Self>
where
    Self: serde::Serialize,
    for<'de> Self: serde::Deserialize<'de>,
{
    type Err: Error;
    fn from_file<P: AsRef<Path>>(path: P) -> std::result::Result<Self, Self::Err>;
}
