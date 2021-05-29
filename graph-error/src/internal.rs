use crate::{AsRes, GraphFailure, GraphResult};

#[derive(Debug, Snafu)]
pub enum GraphRsError {
    #[snafu(display("Download directory does not exist: {}", dir))]
    DownloadDirNoExists { dir: String },
    #[snafu(display(
        "Download file already exists: {}. \
         If you want to over write this file then use overwrite_existing_file(true)",
        name
    ))]
    DownloadFileExists { name: String },
    #[snafu(display("Could not determine file name or the file name exceeded 255 characters"))]
    DownloadFileName,
    #[snafu(display("File name has invalid characters. Must be UTF-8"))]
    FileNameInvalidUTF8,
    #[snafu(display("Missing or invalid: Error: {}", msg))]
    InvalidOrMissing { msg: String },
    #[snafu(display("Invalid file extension. Requires {} but found {}", requires, found))]
    InvalidFileExtension { requires: String, found: String },
}

impl AsRes for GraphRsError {
    fn err_res<T>(self) -> GraphResult<T> {
        GraphFailure::internal(self).err_res()
    }

    fn graph_failure(self) -> GraphFailure {
        GraphFailure::internal(self)
    }
}
