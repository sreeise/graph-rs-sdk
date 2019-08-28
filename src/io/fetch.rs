use std::collections::VecDeque;
use std::convert::TryFrom;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::path::PathBuf;
use reqwest::Client;
use graph_error::GraphError;
use graph_error::GraphFailure;
use crate::drive::event::DownloadFormat;
use crate::drive::pipelines::request::Request;
use crate::drive::ItemResult;
use crate::io::iotools::IoTools;
use reqwest::Response;

/// The FetchBuilder provides an abstraction for downloading files.
pub struct FetchBuilder {
    client: Client,
    path: PathBuf,
    token: String,
    target_url: String,
    file_name: Option<OsString>,
    extension: Option<String>,
}

impl FetchBuilder {
    pub fn new(target_url: &str, path: PathBuf, token: &str) -> FetchBuilder {
        FetchBuilder {
            client: Request::<()>::client().unwrap(),
            path,
            token: token.into(),
            target_url: target_url.into(),
            file_name: None,
            extension: None,
        }
    }

    pub fn rename(&mut self, value: OsString) -> &mut Self {
        self.file_name = Some(value);
        self
    }

    pub fn ext(&mut self, value: &str) -> &mut Self {
        self.extension = Some(value.into());
        self
    }

    pub fn format(&mut self, format: DownloadFormat) -> &mut Self {
        self.ext(format.as_ref())
    }

    pub fn fetch(&mut self) -> ItemResult<PathBuf> {
        self.response(self.path.clone())
    }

    fn parse_content_disposition(&mut self, header: &str) -> Option<OsString> {
        let mut v: Vec<&str> = header.split(";").collect();
        v.retain(|s| !s.is_empty());

        // The filename* indicates that the filename is encoded
        if let Some(value) = v.iter().find(|s| s.starts_with("filename*=utf-8''")) {
            let s = value.replace("filename*=utf-8''", "");
            if let Ok(s) = url::percent_encoding::percent_decode(s.as_bytes()).decode_utf8() {
                return Some(OsString::from(s.to_string()));
            }
        }

        if let Some(value) = v.last() {
            if value.starts_with("filename=") {
                let s = value.replace("\"", "");
                let s = s.replace("filename=", "");
                #[cfg(windows)]
                {
                    use std::os::windows::prelude::*;
                    let os_str = OsStr::new(s.as_ref());
                    return os_str.encode_wide().collect();
                }

                return Some(OsString::from(s.to_string()));
            }
        }
        None
    }

    fn response<P: AsRef<Path>>(&mut self, directory: P) -> ItemResult<PathBuf> {
        // Create the directory if it does not exist.
        IoTools::create_dir(&directory)?;

        let mut response = self
            .client
            .get(self.target_url.as_str())
            .bearer_auth(self.token.as_str())
            .send()?;

        let status = response.status().as_u16();
        if GraphError::is_error(status) {
            return Err(GraphFailure::from(
                GraphError::try_from(&mut response).unwrap_or_default(),
            ));
        }

        // If a filename was specified beforehand.
        if let Some(name) = self.file_name.as_ref() {
            if name.len() <= 255 {
                let path = directory.as_ref().join(name);
                return self.finish((path, response));
            }
        }

        // The content-disposition header, if available, may include the
        // filename either in its normal form or percent encoded.
        if let Some(value) = response.headers().get("content-disposition") {
            if let Ok(s) = std::str::from_utf8(value.as_ref()) {
                if let Some(name) = self.parse_content_disposition(s) {
                    if name.len() <= 255 {
                        let path = directory.as_ref().join(name);
                        return self.finish((path, response));
                    }
                }
            }
        }

        // This is a last ditch effort to find the file name and it
        // may not be the correct one.
        if let Some(name) = response
            .url()
            .path_segments()
            .and_then(std::iter::Iterator::last)
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
        {
            if name.len() <= 255 {
                let path = directory.as_ref().join(name);
                return self.finish((path, response));
            }
        }

        Err(GraphFailure::none_err(
            "Could not determine file name or the file name exceeded 255 characters",
        ))
    }

    fn finish(&mut self, values: (PathBuf, Response)) -> ItemResult<PathBuf> {
        if let Some(ext) = self.extension.as_ref() {
            values.0.with_extension(ext.as_str());
        }
        IoTools::copy(values)
    }
}

// The size of each byte range must be a multiple of 320 KiB (327,680 bytes).
static RANGE_MULTIPLES: [usize; 32] = [
    2, 4, 5, 8, 10, 16, 20, 32, 40, 64, 80, 128, 160, 256, 320, 512, 640, 1024, 1280, 2048, 2560,
    4096, 5120, 8192, 10240, 16384, 20480, 32768, 40960, 65536, 81920, 163_840,
];

/// Get the byte ranges for a given file.
pub struct ByteRange {
    file: OsString,
    byte_ranges: VecDeque<(u64, u64, Vec<u8>)>,
}

impl ByteRange {
    pub fn new<P: AsRef<Path>>(file: P) -> ByteRange {
        ByteRange {
            file: OsString::from(file.as_ref()),
            byte_ranges: Default::default(),
        }
    }

    pub fn file_size(&self) -> std::io::Result<u64> {
        let metadata = fs::metadata(self.file.as_os_str())?;
        Ok(metadata.len())
    }

    pub fn read_from(mut self, from: u64) -> std::io::Result<VecDeque<(u64, u64, Vec<u8>)>> {
        let byte_range = self.find_byte_range()?;
        let size = self.file_size()?;
        let mut f = File::open(self.file.as_os_str())?;

        let mut end = 0;
        let mut counter = from;
        while counter < size {
            let from_pos = counter;

            let mut v: Vec<u8> = Vec::new();
            f.seek(SeekFrom::Start(from_pos))?;
            if size - counter < byte_range {
                counter += size - counter;
                {
                    let reference = f.by_ref();
                    reference.take(counter).read_to_end(&mut v)?;
                }
            } else {
                counter += byte_range;
                {
                    let reference = f.by_ref();
                    reference.take(byte_range).read_to_end(&mut v)?;
                }
            }

            let range = end + v.len() as u64;
            if end != 0 && range < size {
                end += 1;
            }

            let start = end;
            if range > size {
                end += size - end;
            } else {
                end += (v.len() as u64) - 1;
            }
            self.byte_ranges.push_back((start, end, v));
        }
        Ok(self.byte_ranges)
    }

    pub fn read_to_vec(self) -> std::io::Result<VecDeque<(u64, u64, Vec<u8>)>> {
        self.read_from(0)
    }

    fn find_byte_range(&mut self) -> std::io::Result<u64> {
        let file_size = self.file_size()?;
        for (i, next) in RANGE_MULTIPLES.iter().rev().enumerate() {
            let n = *next as u64;
            let div = file_size / n;
            let half = file_size / 2;

            if div > 2 && div < half {
                let num: u64 = file_size / n;
                if num > 15 || i > 25 {
                    return Ok(n);
                }
            }
        }

        Ok(1)
    }
}
