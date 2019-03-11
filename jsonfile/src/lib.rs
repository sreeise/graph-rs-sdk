#[macro_use]
extern crate serde_derive;
use serde_json;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;
use transform_request::RequestError;

#[derive(Serialize, Deserialize)]
pub struct JsonFile;

impl JsonFile {
    fn is_json_ext<P: AsRef<Path>>(path: P) -> bool {
        let extension = path.as_ref().extension();
        extension == Some(OsStr::new("json"))
    }

    /// Writes a data struct to a file given. If the file has already been created this
    /// method will delete the previous file and recreate it with the new struct.
    /// # Arguments
    ///
    /// * `path` - Path to a file and the file name itself.
    /// * `data_struct` - The struct to write to a file in json format.
    pub fn json_file<T, P: AsRef<Path>>(
        path: P,
        data_struct: &T,
    ) -> std::result::Result<(), RequestError>
    where
        T: serde::Serialize,
    {
        if !JsonFile::is_json_ext(&path) {
            return Err(RequestError::error_kind(
                ErrorKind::InvalidData,
                "File does not have a JSON extension",
            ));
        }
        JsonFile::remove_file(&path)?;
        let mut file = OpenOptions::new().create(true).write(true).open(&path)?;

        let serialized = serde_json::to_string(data_struct)?;
        file.write_all(serialized.as_bytes())?;
        file.sync_all()?;
        Ok(())
    }

    /// Writes the struct given, data_struct, to a new JSON file.
    /// Panics if the file is already created.
    pub fn new_json_file<T, P: AsRef<Path>>(
        path: P,
        data_struct: &T,
    ) -> std::result::Result<(), RequestError>
    where
        T: serde::Serialize,
    {
        if !JsonFile::is_json_ext(&path) {
            return Err(RequestError::error_kind(
                ErrorKind::InvalidData,
                "File does not have a JSON extension",
            ));
        }
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)?;

        let serialized = serde_json::to_string(&data_struct)?;
        file.write_all(serialized.as_bytes())?;
        file.sync_all()?;
        Ok(())
    }

    /// Returns a data structure form a file.
    pub fn from_file<T, P: AsRef<Path>>(path: P) -> std::result::Result<T, RequestError>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        if !JsonFile::is_json_ext(&path) {
            return Err(RequestError::error_kind(
                ErrorKind::InvalidData,
                "File does not have a JSON extension",
            ));
        }
        let f = File::open(path)?;
        let self_as_json = serde_json::from_reader(f)?;
        Ok(self_as_json)
    }

    /// Returns a vec of data structures for every file in a directory. All files
    /// in the directory given should be a serialized data structure of the same type.
    pub fn from_dir_as_vec<T, P: AsRef<Path>>(dir: P) -> std::result::Result<Vec<T>, RequestError>
    where
        std::string::String: std::convert::From<P>,
        for<'de> T: serde::Deserialize<'de>,
    {
        let paths = fs::read_dir(&dir)?;
        let mut t_vec = Vec::new();

        for path in paths {
            let path_buf = path?.path();

            let p = match path_buf.to_str() {
                Some(t) => t,
                None => {
                    return Err(RequestError::error_kind(
                        ErrorKind::NotFound,
                        "Path not found.",
                    ))
                },
            };

            let t: T = JsonFile::from_file(p)?;
            t_vec.push(t);
        }

        Ok(t_vec)
    }

    fn remove_file<P: AsRef<Path>>(path: P) -> std::result::Result<(), RequestError> {
        if path.as_ref().exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}
