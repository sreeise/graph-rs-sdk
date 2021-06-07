#[derive(Debug, thiserror::Error)]
pub enum GraphRsError {
    #[error("Download directory does not exist: {dir}")]
    DownloadDirNoExists { dir: String },
    #[error(
        "Download file already exists: {name}. \
         If you want to over write this file then use overwrite_existing_file(true)"
    )]
    DownloadFileExists { name: String },
    #[error("Could not determine file name or the file name exceeded 255 characters")]
    DownloadFileName,
    #[error("File name has invalid characters. Must be UTF-8")]
    FileNameInvalidUTF8,
    #[error("Missing or invalid: Error: {msg}")]
    InvalidOrMissing { msg: String },
    #[error("Invalid file extension. Requires {requires} but found {found}")]
    InvalidFileExtension { requires: String, found: String },
}
