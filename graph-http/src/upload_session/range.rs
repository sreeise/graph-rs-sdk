use crate::traits::ByteRangeMultiple;
use bytes::{Buf, BytesMut};
use graph_error::{GraphFailure, GraphResult};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_RANGE, CONTENT_TYPE};
use std::collections::VecDeque;
use std::io::Read;
use tokio::io::AsyncReadExt;

#[derive(Clone, Debug, Default)]
pub(crate) struct Range {
    pub(crate) start_pos: u64,
    pub(crate) end_pos: u64,
    pub(crate) bytes: Vec<u8>,
}

impl Range {
    pub fn start(&self) -> u64 {
        self.start_pos
    }

    pub fn end(&self) -> u64 {
        self.end_pos
    }

    pub fn body(self) -> Vec<u8> {
        self.bytes
    }

    pub fn content_length(&self) -> u64 {
        (self.end_pos - self.start_pos) + 1
    }

    pub fn content_range(&self, size: u64) -> String {
        format!("bytes {}-{}/{}", self.start(), self.end(), size)
    }
}

#[derive(Debug, Default)]
pub(crate) struct RangeIter {
    size: u64,
    dequeue: VecDeque<Range>,
}

impl RangeIter {
    pub fn new(size: u64) -> RangeIter {
        RangeIter {
            size,
            dequeue: VecDeque::new(),
        }
    }

    pub fn from_reader<T: Read>(mut reader: T) -> GraphResult<RangeIter> {
        let mut buf: Vec<u8> = Vec::new();
        let _ = reader.read_to_end(&mut buf)?;
        RangeIter::try_from(BytesMut::from_iter(buf.iter()))
    }

    pub async fn from_async_read<T: AsyncReadExt + Unpin>(mut reader: T) -> GraphResult<RangeIter> {
        let mut buf: Vec<u8> = Vec::new();
        let _ = reader.read_to_end(&mut buf).await?;
        RangeIter::try_from(BytesMut::from_iter(buf.iter()))
    }

    pub fn pop_front(&mut self) -> Option<(HeaderMap, reqwest::Body)> {
        let range = self.dequeue.pop_front()?;

        let content_range = range.content_range(self.size);
        let content_length = range.content_length().to_string();
        let mut header_map = HeaderMap::new();

        header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        header_map.insert(
            CONTENT_LENGTH,
            HeaderValue::from_str(content_length.as_str()).ok()?,
        );
        header_map.insert(
            CONTENT_RANGE,
            HeaderValue::from_str(content_range.as_str()).ok()?,
        );

        Some((header_map, reqwest::Body::from(range.body())))
    }
}

impl Iterator for RangeIter {
    type Item = Range;

    fn next(&mut self) -> Option<Self::Item> {
        self.dequeue.pop_front()
    }
}

impl TryFrom<BytesMut> for RangeIter {
    type Error = GraphFailure;

    fn try_from(bytes_mut: BytesMut) -> GraphResult<Self> {
        let multiple = bytes_mut.byte_range_multiple();
        let size = bytes_mut.len() as u64;

        let mut range_iter = RangeIter::new(size);
        let mut reader = bytes_mut.reader();

        let mut end = 0;
        let mut counter = 0;

        while counter < size {
            let mut buf: Vec<u8>;
            if size - counter > multiple {
                counter += multiple;
                buf = vec![0u8; multiple as usize];
                reader.read_exact(&mut buf)?;
            } else {
                let last = size - counter;
                buf = vec![0u8; last as usize];
                counter += size - counter;
                reader.read_exact(&mut buf)?;
            }

            let range = end + (buf.len() as u64);
            if range < size && end != 0 {
                end += 1;
            }

            let start = end;
            if range > size {
                end += size - end;
            } else {
                end += (buf.len() as u64) - 1;
            }

            range_iter.dequeue.push_back(Range {
                start_pos: start,
                end_pos: end,
                bytes: buf,
            })
        }

        Ok(range_iter)
    }
}
