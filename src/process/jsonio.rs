use serde_json;
use std;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct JsonFile;

impl JsonFile {
    /// Writes a data struct to a file given. If the file has already been created this
    /// method will delete the previous file and recreate it with the new struct.
    /// # Arguments
    ///
    /// * `path` - Path to a file and the file name itself.
    /// * `data_struct` - The struct to write to a file in json format.
    pub fn json_file<T, P: AsRef<Path>>(path: P, data_struct: &T) -> std::io::Result<()>
    where
        T: serde::Serialize,
    {
        if Path::new(path.as_ref()).exists() {
            JsonFile::remove_file(&path).unwrap();
        }
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&path)
            .expect("Could not write to file");

        let serialized = serde_json::to_string(data_struct).expect("Error serializing struct");
        file.write_all(serialized.as_bytes())?;
        file.sync_all()?;
        Ok(())
    }

    /// Writes the struct given, data_struct, to a new JSON file.
    /// Panics if the file is already created.
    pub fn new_json_file<T, P: AsRef<Path>>(path: P, data_struct: &T) -> std::io::Result<()>
    where
        T: serde::Serialize,
    {
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)
            .map_err(|error| {
                if error.kind() == ErrorKind::NotFound {
                    File::create(&path).unwrap_or_else(|error| {
                        panic!(
                            "The file was originally not found but an error occurred: {:?}",
                            error
                        );
                    });
                }
            })
            .expect("Could not write to file");

        let serialized = serde_json::to_string(&data_struct).expect("Error serializing struct");
        file.write_all(serialized.as_bytes())?;
        file.sync_all()?;
        Ok(())
    }

    /// Returns a data structure form a file.
    pub fn from_file<T, P: AsRef<Path>>(path: P) -> std::io::Result<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let f = File::open(path)?;
        let self_as_json = serde_json::from_reader(f)?;
        Ok(self_as_json)
    }

    /// Returns a vec of data structures for every file in a directory. All files
    /// in the directory given should be a serialized data structure of the same type.
    pub fn from_dir_as_vec<T, P: AsRef<Path>>(dir: P) -> std::io::Result<Vec<T>>
    where
        std::string::String: std::convert::From<P>,
        for<'de> T: serde::Deserialize<'de>,
    {
        let paths = fs::read_dir(&dir)?;
        let mut t_vec = Vec::new();

        for path in paths {
            let struct_from_file: T = JsonFile::from_file(
                path.expect("could not construct original path")
                    .path()
                    .to_str()
                    .expect("could not get str from path"),
            )?;
            t_vec.push(struct_from_file);
        }

        Ok(t_vec)
    }

    fn remove_file<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
        if path.as_ref().exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}
