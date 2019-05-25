use std::error::Error;
use std::path::Path;

pub trait FromToFile<RHS = Self>
where
    Self: serde::Serialize,
    for<'de> Self: serde::Deserialize<'de>,
{
    type Err: Error;
    type Output;

    /// Writes a data struct to a json file given. The struct must implement
    /// serde_derive Serialize and Deserialize.
    /// # Arguments
    ///
    /// * `path` - Path to a file and the file name itself.
    fn to_json_file<P: AsRef<Path>>(&self, path: P)
        -> std::result::Result<Self::Output, Self::Err>;

    /// Returns a struct from a given json file. The struct must implement
    /// serde_derive Serialize and Deserialize.
    /// # Arguments
    ///
    /// * `path` - Path to a file and the file name itself.
    fn from_json_file<P: AsRef<Path>>(path: P) -> std::result::Result<Self, Self::Err>;

    /// Writes a data struct to a given yaml file. The struct must implement
    /// serde_derive Serialize and Deserialize.
    /// # Arguments
    ///
    /// * `path` - Path to a file and the file name itself.
    fn to_yaml_file<P: AsRef<Path>>(&self, path: P)
        -> std::result::Result<Self::Output, Self::Err>;

    /// Returns a struct from a given yaml file. The struct must implement
    /// serde_derive Serialize and Deserialize.
    /// # Arguments
    ///
    /// * `path` - Path to a file and the file name itself.
    fn from_yaml_file<P: AsRef<Path>>(path: P) -> std::result::Result<Self, Self::Err>;
}
