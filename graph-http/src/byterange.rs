use crate::traits::{AsyncByteRangeRead, AsyncTryFrom, ByteRangeMultiple, ByteRangeRead};
use async_trait::async_trait;
use graph_error::{GraphFailure, GraphResult};
use tokio::io::AsyncSeekExt;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};
use std::io::{ErrorKind, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use tokio::io::{AsyncReadExt, AsyncSeekExt};

pub struct ByteRanges<F> {
    file: F,
    file_size: u64,
}

impl<F> Debug for ByteRanges<F>
where
    F: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ByteRanges<F>")
            .field("file_size", &self.file_size)
            .field("file", &self.file)
            .finish()
    }
}

impl<F> ByteRangeMultiple for ByteRanges<F> {
    fn file_size(&self) -> u64 {
        self.file_size
    }
}

impl ByteRanges<std::fs::File> {
    pub fn new<P: AsRef<Path>>(path: P) -> GraphResult<ByteRanges<std::fs::File>> {
        let file = std::fs::File::open(path)?;
        let file_size = file.metadata()?.len();
        Ok(ByteRanges { file, file_size })
    }
}

impl ByteRangeRead for ByteRanges<std::fs::File> {
    fn read_to_vec(mut self) -> std::io::Result<VecDeque<(u64, u64, Vec<u8>)>> {
        let byte_range = self.byte_range_multiples();
        let size = self.file_size();
        let mut byte_range_queue: VecDeque<(u64, u64, Vec<u8>)> = VecDeque::new();

        let mut end = 0;
        let mut counter = 0;
        while counter < size {
            let from_pos = counter;

            let mut v: Vec<u8> = Vec::new();
            self.file.seek(SeekFrom::Start(from_pos))?;
            if size - counter < byte_range {
                counter += size - counter;
                {
                    let reference = self.file.by_ref();
                    reference.take(counter).read_to_end(&mut v)?;
                }
            } else {
                counter += byte_range;
                {
                    let reference = self.file.by_ref();
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
            byte_range_queue.push_back((start, end, v));
        }
        Ok(byte_range_queue)
    }

    fn read_to_vec_range(
        mut self,
        start: u64,
        end: u64,
    ) -> std::io::Result<VecDeque<(u64, u64, Vec<u8>)>> {
        let size = self.file_size();
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

        let mut byte_range_queue: VecDeque<(u64, u64, Vec<u8>)> = VecDeque::new();
        let byte_range = end - start;
        let mut end = end;
        let mut counter = start;

        while counter < size {
            let from_pos = counter;
            let mut v: Vec<u8> = Vec::new();
            self.file.seek(SeekFrom::Start(from_pos))?;

            if size - counter < byte_range {
                counter += size - counter;
                self.file.by_ref().take(counter).read_to_end(&mut v)?;
            } else {
                counter += byte_range;
                self.file.by_ref().take(byte_range).read_to_end(&mut v)?;
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
            byte_range_queue.push_back((start, end, v));
        }
        Ok(byte_range_queue)
    }
}

impl ByteRanges<tokio::fs::File> {
    pub async fn new<P: AsRef<Path>>(path: P) -> GraphResult<ByteRanges<tokio::fs::File>> {
        let file = tokio::fs::File::open(path).await?;
        let file_size = file.metadata().await?.len();
        Ok(ByteRanges { file, file_size })
    }
}

#[async_trait]
impl AsyncByteRangeRead for ByteRanges<tokio::fs::File> {
    async fn read_to_vec(mut self) -> std::io::Result<VecDeque<(u64, u64, Vec<u8>)>> {
        let range_multiple = self.byte_range_multiples();
        let size = self.file_size();
        let mut byte_range_queue: VecDeque<(u64, u64, Vec<u8>)> = VecDeque::new();

        let mut end = 0;
        let mut counter = 0;
        while counter < size {
            let from_pos = counter;

            let mut v: Vec<u8>;
            self.file.seek(SeekFrom::Start(from_pos)).await?;
            if size - counter < range_multiple {
                let last = size - counter;
                v = vec![0u8; last as usize];
                counter += size - counter;
                self.file.read_exact(&mut v).await?;
            } else {
                counter += range_multiple;
                v = vec![0u8; range_multiple as usize];
                self.file.read_exact(&mut v).await?;
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
            byte_range_queue.push_back((start, end, v));
        }
        Ok(byte_range_queue)
    }

    async fn read_to_vec_range(
        mut self,
        start: u64,
        end: u64,
    ) -> std::io::Result<VecDeque<(u64, u64, Vec<u8>)>> {
        let size = self.file_size;
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

        let mut byte_range_queue: VecDeque<(u64, u64, Vec<u8>)> = VecDeque::new();
        let byte_range = end - start;
        let mut end = end;
        let mut counter = start;

        while counter < size {
            let from_pos = counter;
            let mut v: Vec<u8>;
            self.file.seek(SeekFrom::Start(from_pos)).await?;

            if size - counter < byte_range {
                counter += size - counter;
                v = vec![0u8; counter as usize];
                self.file.read_exact(&mut v).await?;
            } else {
                counter += byte_range;
                v = vec![0u8; byte_range as usize];
                self.file.read_exact(&mut v).await?;
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
            byte_range_queue.push_back((start, end, v));
        }
        Ok(byte_range_queue)
    }
}

#[derive(Debug, Default)]
pub struct ByteRangeIterator {
    file_size: u64,
    byte_range: VecDeque<(u64, u64, Vec<u8>)>,
}

impl ByteRangeIterator {
    pub fn new(file_size: u64, byte_range: VecDeque<(u64, u64, Vec<u8>)>) -> ByteRangeIterator {
        ByteRangeIterator {
            file_size,
            byte_range,
        }
    }

    pub fn from_range<P: AsRef<Path>>(
        start: u64,
        end: u64,
        file: P,
    ) -> GraphResult<ByteRangeIterator> {
        let byte_range: ByteRanges<std::fs::File> = ByteRanges::<std::fs::File>::new(file)?;
        let file_size = byte_range.file_size();
        Ok(ByteRangeIterator::new(
            file_size,
            byte_range.read_to_vec_range(start, end)?,
        ))
    }

    pub async fn async_from_range<P: AsRef<Path>>(
        start: u64,
        end: u64,
        file: P,
    ) -> GraphResult<ByteRangeIterator> {
        let byte_range: ByteRanges<tokio::fs::File> =
            ByteRanges::<tokio::fs::File>::new(file).await?;
        let file_size = byte_range.file_size();
        Ok(ByteRangeIterator::new(
            file_size,
            byte_range.read_to_vec_range(start, end).await?,
        ))
    }

    pub async fn try_from_async(value: PathBuf) -> GraphResult<ByteRangeIterator> {
        let byte_range: ByteRanges<tokio::fs::File> =
            ByteRanges::<tokio::fs::File>::new(value).await?;
        let file_size = byte_range.file_size();
        let bytes = byte_range.read_to_vec().await?;
        Ok(ByteRangeIterator::new(file_size, bytes))
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

impl Iterator for ByteRangeIterator {
    type Item = (Vec<u8>, u64, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}

impl TryFrom<PathBuf> for ByteRangeIterator {
    type Error = GraphFailure;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let byte_range = ByteRanges::<std::fs::File>::new(value)?;
        let file_size = byte_range.file_size();
        let bytes = byte_range.read_to_vec()?;
        Ok(ByteRangeIterator::new(file_size, bytes))
    }
}

#[async_trait]
impl AsyncTryFrom<PathBuf> for ByteRangeIterator {
    type Error = GraphFailure;

    async fn async_try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let byte_range = ByteRanges::<tokio::fs::File>::new(value).await?;
        let file_size = byte_range.file_size();
        let bytes = byte_range.read_to_vec().await?;
        Ok(ByteRangeIterator::new(file_size, bytes))
    }
}
