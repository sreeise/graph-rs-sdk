use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

/// Config for downloading files using Microsoft Graph File and OneDrive APIs.
#[derive(Clone, Debug, Default)]
pub struct FileConfig {
    pub path: PathBuf,
    pub create_directory_all: bool,
    pub overwrite_existing_file: bool,
    pub file_name: Option<OsString>,
    pub extension: Option<OsString>,
}

impl FileConfig {
    /// Create the file configuration for downloading files.
    ///
    /// # Example
    /// ```rust
    /// use std::ffi::{OsStr, OsString};
    /// use std::path::Path;
    /// use graph_http::FileConfig;
    /// let config = FileConfig::new("./examples")
    ///     .create_directories(true)
    ///     .file_name(OsStr::new("example.json"));
    ///
    /// # assert_eq!(Path::new("./examples"), config.path.as_path());
    /// # assert!(config.create_directory_all);
    /// # assert_eq!(Some(&OsString::from("example.json")),  config.file_name.as_ref());
    /// ```
    ///
    /// # Example
    /// ```rust
    /// use std::ffi::{OsStr, OsString};
    /// use std::path::Path;
    /// use graph_http::FileConfig;
    /// let config = FileConfig::new("./examples")
    ///     .overwrite_existing_file(true)
    ///     .extension(OsStr::new("pdf"));
    ///
    /// # assert_eq!(Path::new("./examples"), config.path.as_path());
    /// # assert!(config.overwrite_existing_file);
    /// # assert_eq!(Some(&OsString::from("pdf")),  config.extension.as_ref());
    /// ```
    pub fn new<P: AsRef<Path>>(path: P) -> FileConfig {
        FileConfig {
            path: path.as_ref().to_path_buf(),
            create_directory_all: false,
            overwrite_existing_file: false,
            file_name: None,
            extension: None,
        }
    }

    /// Get a default FileConfig builder.
    ///
    /// Create the file configuration for downloading files.
    ///
    /// # Example
    /// ```rust
    /// use std::ffi::{OsStr, OsString};
    /// use std::path::Path;
    /// use graph_http::FileConfig;
    /// let config = FileConfig::builder()
    ///     .path("./examples")
    ///     .create_directories(true)
    ///     .overwrite_existing_file(true)
    ///     .file_name(OsStr::new("example.pdf"));
    ///
    /// # assert_eq!(Path::new("./examples"), config.path.as_path());
    /// # assert!(config.create_directory_all);
    /// # assert!(config.overwrite_existing_file);
    /// ```
    ///
    /// # Example
    /// ```rust
    /// use std::ffi::{OsStr, OsString};
    /// use std::path::Path;
    /// use graph_http::FileConfig;
    /// let config = FileConfig::builder()
    ///     .path("./examples")
    ///     .overwrite_existing_file(true)
    ///     .extension(OsStr::new("pdf"));
    ///
    /// # assert_eq!(Path::new("./examples"), config.path.as_path());
    /// # assert!(config.overwrite_existing_file);
    /// # assert_eq!(Some(&OsString::from("pdf")),  config.extension.as_ref());
    /// ```
    pub fn builder() -> FileConfig {
        FileConfig {
            path: Default::default(),
            create_directory_all: false,
            overwrite_existing_file: false,
            file_name: None,
            extension: None,
        }
    }

    /// Set the directory path for storing a downloaded file.
    ///
    /// # Example
    /// ```rust
    /// use std::path::Path;
    /// use graph_http::FileConfig;
    /// let config = FileConfig::builder()
    ///     .path("./examples");
    ///
    /// # assert_eq!(Path::new("./examples"), config.path.as_path());
    /// ```
    pub fn path<P: AsRef<Path>>(mut self, path: P) -> FileConfig {
        self.path = path.as_ref().to_path_buf();
        self
    }

    /// Create all directories in the path given if they do not exist.
    ///
    /// # Example
    /// ```rust
    /// use graph_http::FileConfig;
    /// let config = FileConfig::new("./examples")
    ///     .create_directories(true);
    ///
    /// # assert!(config.create_directory_all);
    /// ```
    pub fn create_directories(mut self, create_directories: bool) -> FileConfig {
        self.create_directory_all = create_directories;
        self
    }

    /// Overwrite an existing file of the same file name and path.
    ///
    /// # Example
    /// ```rust
    /// use graph_http::FileConfig;
    /// let config = FileConfig::new("./examples")
    ///     .overwrite_existing_file(true);
    ///
    /// # assert!(config.overwrite_existing_file);
    /// ```
    pub fn overwrite_existing_file(mut self, overwrite_file: bool) -> FileConfig {
        self.overwrite_existing_file = overwrite_file;
        self
    }

    /// Set the file name and extension. If you use the format ODataQuery to change
    /// the file type such as from JSON to PDF then you must provide a name for the
    /// file or the download will fail.
    ///
    /// # Example
    /// ```rust
    /// use std::ffi::{OsStr, OsString};
    /// use graph_http::FileConfig;
    ///
    /// let config = FileConfig::new("./examples")
    ///     .file_name(OsStr::new("example.pdf"));
    ///
    /// # assert_eq!(Some(&OsString::from("example.pdf")),  config.file_name.as_ref());
    /// ```
    pub fn file_name(mut self, file_name: &OsStr) -> FileConfig {
        self.file_name = Some(file_name.to_os_string());
        self
    }

    /// Set the extension of the file being downloaded. This only changes the extension name
    /// of the file and does not change the format of the file. You can change the format of
    /// the file using the ODataQuery format method such as changing from JSONto PDF
    ///
    /// # Example
    /// ```rust
    /// use std::ffi::{OsStr, OsString};
    /// use graph_http::FileConfig;
    ///
    /// let config = FileConfig::new("./examples")
    ///     .extension(OsStr::new("pdf"));
    ///
    /// # assert_eq!(Some(&OsString::from("pdf")),  config.extension.as_ref());
    /// ```
    pub fn extension(mut self, ext: &OsStr) -> FileConfig {
        self.extension = Some(ext.to_os_string());
        self
    }

    /// Create all directories in the path given if they do not exist.
    ///
    /// # Example
    /// ```rust
    /// use std::path::Path;
    /// use graph_http::FileConfig;
    /// let mut config = FileConfig::default();
    /// config.set_path("./examples");
    ///
    /// # assert_eq!(Path::new("./examples"), config.path.as_path())
    /// ```
    pub fn set_path<P: AsRef<Path>>(&mut self, path: P) {
        self.path = path.as_ref().to_path_buf();
    }

    /// Create all directories in the path given if they do not exist.
    ///
    /// # Example
    /// ```
    /// use graph_http::FileConfig;
    /// let mut config = FileConfig::new("./examples");
    /// config.set_create_directories(true);
    ///
    /// # assert!(config.create_directory_all);
    /// ```
    pub fn set_create_directories(&mut self, create_directories: bool) {
        self.create_directory_all = create_directories;
    }

    /// Overwrite an existing file of the same file name and path.
    ///
    /// # Example
    /// ```rust
    /// use graph_http::FileConfig;
    /// let mut config = FileConfig::new("./examples");
    /// config.set_overwrite_existing_file(true);
    ///
    /// # assert!(config.overwrite_existing_file);
    /// ```
    pub fn set_overwrite_existing_file(&mut self, overwrite_file: bool) {
        self.overwrite_existing_file = overwrite_file;
    }

    /// Set the file name and extension. If you use the format ODataQuery to change
    /// the file type such as from JSON to PDF then you must provide a name for the
    /// file or the download will fail.
    ///
    /// # Example
    /// ```rust
    /// use std::ffi::{OsStr, OsString};
    /// use graph_http::FileConfig;
    ///
    /// let mut config = FileConfig::new("./examples");
    /// config.set_file_name(OsStr::new("example.pdf"));
    ///
    /// # assert_eq!(Some(&OsString::from("example.pdf")),  config.file_name.as_ref());
    /// ```
    pub fn set_file_name(&mut self, file_name: &OsStr) {
        self.file_name = Some(file_name.to_os_string());
    }

    /// Set the extension of the file being downloaded. This only changes the extension name
    /// of the file and does not change the format of the file. You can change the format of
    /// the file using the ODataQuery format method such as changing from JSONto PDF
    ///
    /// # Example
    /// ```rust
    /// use std::ffi::{OsStr, OsString};
    /// use graph_http::FileConfig;
    ///
    /// let mut config = FileConfig::new("./examples")
    ///     .extension(OsStr::new("pdf"));
    ///
    /// # assert_eq!(Some(&OsString::from("pdf")),  config.extension.as_ref());
    /// ```
    pub fn set_extension(&mut self, ext: &OsStr) {
        self.extension = Some(ext.to_os_string());
    }
}
