use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Clone, Debug, Default)]
pub struct DownloadConfig {
    pub path: PathBuf,
    pub create_directory_all: bool,
    pub overwrite_existing_file: bool,
    pub file_name: Option<OsString>,
    pub extension: Option<String>,
}
