/// Represents the types of file formats that a file can be
/// converted using
///
/// # Example
/// ```rust,ignore
/// # use rust_onedrive::prelude::*;
/// # use std::path::PathBuf;
///
/// let drive: Drive = Drive::new("ACCESS_TOKEN", DriveVersion::V1);
/// let path_buf: PathBuf = drive.download_format("directory", &mut drive::Value, DownloadFormat::PDF).unwrap();
/// println!("{:#?}", path_buf);
/// ```
pub enum DownloadFormat {
    GLB,
    HTML,
    JPG,
    PDF,
}

impl AsRef<str> for DownloadFormat {
    fn as_ref(&self) -> &str {
        match self {
            DownloadFormat::GLB => "glb",
            DownloadFormat::HTML => "html",
            DownloadFormat::JPG => "jpg",
            DownloadFormat::PDF => "pdf",
        }
    }
}
