use serde;
use serde_json;
use std;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct JsonFile;

impl JsonFile {
    /// Writes a data struct to a file given. If the file has already been created this
    /// method will truncate previous data and only write the new struct.
    pub fn json_file<T, P: AsRef<Path>>(path: P, data_struct: &T) -> io::Result<()>
    where
        T: serde::Serialize,
    {
        let mut file = File::create(path)?;
        let serialized = serde_json::to_string(data_struct)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    /// Writes the struct given, data_struct, to a new JSON file.
    /// Panics if the file is already created.
    pub fn new_json_file<T, P: AsRef<Path>>(path: P, data_struct: &T) -> io::Result<()>
    where
        T: serde::Serialize,
    {
        OpenOptions::new()
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

        let serialized = serde_json::to_string(data_struct).expect("Error serializing struct");
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .expect("Error writing file with serialized struct");
        file.write_all(serialized.as_bytes())
            .expect("Could not write AuthFlow as a new json file");
        Ok(())
    }

    /// Returns a data structure form a file.
    pub fn from_file<T, P: AsRef<Path>>(path: P) -> io::Result<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let f = File::open(path)?;
        let self_as_json = serde_json::from_reader(f)?;
        Ok(self_as_json)
    }

    /// Returns a vec of data structures for every file in a directory. All files
    /// in the directory given should be a serialized data structure of the same type.
    pub fn from_dir_as_vec<T, P: AsRef<Path>>(dir: P) -> io::Result<Vec<T>>
    where
        std::string::String: std::convert::From<P>,
        for<'de> T: serde::Deserialize<'de>,
    {
        let paths = fs::read_dir(&dir)?;
        let initial_path = String::from(dir);
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
}
