use bytes::BytesMut;
use graph_error::GraphFailure;
use std::ffi::{OsStr, OsString};
use std::io::Read;
use std::path::{Path, PathBuf};

/// Config for downloading files using Microsoft Graph File and OneDrive APIs.
/// FileConfig can also be used for uploads but all fields except for the provided
/// path are ignored.
///
/// # Example
/// ```rust
/// use std::ffi::{OsStr, OsString};
/// use std::path::Path;
/// use graph_http::api_impl::FileConfig;
///
/// let config = FileConfig::new("./examples")
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
/// use graph_http::api_impl::FileConfig;
///
/// let config = FileConfig::new("./examples")
///     .overwrite_existing_file(false)
///     .extension(OsStr::new("pdf"));
///
/// # assert_eq!(Path::new("./examples"), config.path.as_path());
/// # assert!(config.overwrite_existing_file);
/// # assert_eq!(Some(&OsString::from("pdf")),  config.extension.as_ref());
/// ```
#[derive(Clone, Debug, Default)]
pub struct FileConfig {
    pub path: PathBuf,
    /// Used only when downloading files. Default is true.
    pub create_directory_all: bool,
    /// Used only when downloading files
    pub overwrite_existing_file: bool,
    /// Used only when downloading files
    pub file_name: Option<OsString>,
    /// Used only when downloading files
    pub extension: Option<OsString>,
}

impl FileConfig {
    /// Create the file configuration for downloading files.
    ///
    /// # Example
    /// ```rust
    /// use std::ffi::{OsStr, OsString};
    /// use std::path::Path;
    /// use graph_http::api_impl::FileConfig;
    ///
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
    /// use graph_http::api_impl::FileConfig;
    ///
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
            create_directory_all: true,
            overwrite_existing_file: false,
            file_name: None,
            extension: None,
        }
    }

    /// Create all directories in the path given if they do not exist.
    ///
    /// # Example
    /// ```rust
    /// use graph_http::api_impl::FileConfig;
    ///
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
    /// This is true by default.
    ///
    /// # Example
    /// ```rust
    /// use graph_http::api_impl::FileConfig;
    ///
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
    /// use graph_http::api_impl::FileConfig;
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
    /// use graph_http::api_impl::FileConfig;
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
    /// use graph_http::api_impl::FileConfig;
    ///
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
    /// use graph_http::api_impl::FileConfig;
    ///
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
    /// use graph_http::api_impl::FileConfig;
    ///
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
    /// use graph_http::api_impl::FileConfig;
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
    /// use graph_http::api_impl::FileConfig;
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

impl From<PathBuf> for FileConfig {
    fn from(path_buf: PathBuf) -> Self {
        FileConfig::new(path_buf.as_path())
    }
}

impl From<&Path> for FileConfig {
    fn from(path: &Path) -> Self {
        FileConfig::new(path)
    }
}

impl TryFrom<FileConfig> for BytesMut {
    type Error = GraphFailure;

    fn try_from(file_config: FileConfig) -> Result<Self, Self::Error> {
        let mut file = std::fs::File::open(file_config.path.as_path())?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(BytesMut::from_iter(buf))
    }
}
