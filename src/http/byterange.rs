use std::collections::VecDeque;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
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
