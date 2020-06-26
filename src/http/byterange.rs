use from_as::TryFrom;
use graph_error::{GraphFailure, GraphResult};
use std::collections::VecDeque;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

// The size of each byte range must be a multiple of 320 KiB (327,680 bytes).
static RANGE_MULTIPLES: [usize; 32] = [
    2, 4, 5, 8, 10, 16, 20, 32, 40, 64, 80, 128, 160, 256, 320, 512, 640, 1024, 1280, 2048, 2560,
    4096, 5120, 8192, 10240, 16384, 20480, 32768, 40960, 65536, 81920, 163_840,
];

/// Get the byte ranges for a given file.
pub struct ByteRange {
    file: PathBuf,
    byte_ranges: VecDeque<(u64, u64, Vec<u8>)>,
}

impl ByteRange {
    pub fn new<P: AsRef<Path>>(file: P) -> ByteRange {
        ByteRange {
            file: file.as_ref().to_path_buf(),
            byte_ranges: Default::default(),
        }
    }

    pub fn file_size(&self) -> std::io::Result<u64> {
        let metadata = fs::metadata(self.file.as_os_str())?;
        Ok(metadata.len())
    }

    pub fn read_to_vec_range(
        mut self,
        start: u64,
        end: u64,
    ) -> std::io::Result<VecDeque<(u64, u64, Vec<u8>)>> {
        let size = self.file_size()?;
        if start > size || end > size {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "start or end range is greater than file size",
            ));
        } else if end <= start {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "start range cannot be greater than or equal to end range",
            ));
        }

        let byte_range = end - start;
        let mut f = File::open(self.file.as_os_str())?;
        let mut end = end;
        let mut counter = start;

        while counter < size {
            let from_pos = counter;
            let mut v: Vec<u8> = Vec::new();
            f.seek(SeekFrom::Start(from_pos))?;

            if size - counter < byte_range {
                counter += size - counter;
                {
                    f.by_ref().take(counter).read_to_end(&mut v)?;
                }
            } else {
                counter += byte_range;
                {
                    f.by_ref().take(byte_range).read_to_end(&mut v)?;
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

    pub fn read_to_vec(mut self) -> std::io::Result<VecDeque<(u64, u64, Vec<u8>)>> {
        let byte_range = self.find_byte_range()?;
        let size = self.file_size()?;
        let mut f = File::open(self.file.as_os_str())?;

        let mut end = 0;
        let mut counter = 0;
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

#[derive(Debug, Default)]
pub struct HttpByteRange {
    file_size: u64,
    byte_range: VecDeque<(u64, u64, Vec<u8>)>,
}

impl HttpByteRange {
    pub fn new(file_size: u64, byte_range: VecDeque<(u64, u64, Vec<u8>)>) -> HttpByteRange {
        HttpByteRange {
            file_size,
            byte_range,
        }
    }

    pub fn from_range<P: AsRef<Path>>(start: u64, end: u64, file: P) -> GraphResult<HttpByteRange> {
        let path: &Path = file.as_ref();
        let byte_range = ByteRange::new(path.to_path_buf());
        let file_size = byte_range.file_size()?;
        Ok(HttpByteRange::new(
            file_size,
            byte_range.read_to_vec_range(start, end)?,
        ))
    }

    pub async fn async_from_range<P: AsRef<Path>>(
        start: u64,
        end: u64,
        file: P,
    ) -> std::io::Result<HttpByteRange> {
        let path: &Path = file.as_ref();
        let byte_range = ByteRange::new(path.to_path_buf());
        let file_size = byte_range.file_size()?;
        Ok(HttpByteRange::new(
            file_size,
            byte_range.read_to_vec_range(start, end)?,
        ))
    }

    pub async fn try_from_async(value: PathBuf) -> Result<HttpByteRange, GraphFailure> {
        let byte_range = ByteRange::new(value);
        let file_size = byte_range.file_size()?;
        let bytes = byte_range.read_to_vec()?;
        Ok(HttpByteRange::new(file_size, bytes))
    }

    pub fn file_size(&self) -> u64 {
        self.file_size
    }

    pub fn is_empty(&self) -> bool {
        self.byte_range.is_empty()
    }

    pub fn pop_front(&mut self) -> Option<(Vec<u8>, u64, String)> {
        let next = self.byte_range.pop_front()?;
        let mut content_length = next.1 - next.0;
        content_length += 1;
        let start = next.0.to_string();
        let end = next.1.to_string();
        let file_size = self.file_size.to_string();
        Some((
            next.2,
            content_length,
            format!(
                "bytes {}-{}/{}",
                start.as_str(),
                end.as_str(),
                file_size.as_str()
            ),
        ))
    }

    pub fn clear(&mut self) {
        self.byte_range.clear();
    }
}

impl Iterator for HttpByteRange {
    type Item = (Vec<u8>, u64, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}

impl TryFrom<PathBuf> for HttpByteRange {
    type Error = GraphFailure;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let byte_range = ByteRange::new(value);
        let file_size = byte_range.file_size()?;
        let bytes = byte_range.read_to_vec()?;
        Ok(HttpByteRange::new(file_size, bytes))
    }
}
