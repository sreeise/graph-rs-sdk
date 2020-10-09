use async_trait::async_trait;
use std::collections::VecDeque;

static RANGE_MULTIPLES: [usize; 32] = [
    2, 4, 5, 8, 10, 16, 20, 32, 40, 64, 80, 128, 160, 256, 320, 512, 640, 1024, 1280, 2048, 2560,
    4096, 5120, 8192, 10240, 16384, 20480, 32768, 40960, 65536, 81920, 163_840,
];

pub trait ByteRangeMultiple {
    fn file_size(&self) -> u64;

    fn byte_range_multiples(&self) -> u64 {
        let file_size = self.file_size();
        for (i, next) in RANGE_MULTIPLES.iter().rev().enumerate() {
            let n = *next as u64;
            let div = file_size / n;
            let half = file_size / 2;

            if div > 2 && div < half {
                let num: u64 = file_size / n;
                if num > 15 || i > 25 {
                    return n;
                }
            }
        }
        1
    }
}

pub trait ByteRangeRead: ByteRangeMultiple {
    fn read_to_vec(self) -> std::io::Result<VecDeque<(u64, u64, Vec<u8>)>>;
    fn read_to_vec_range(
        self,
        start: u64,
        end: u64,
    ) -> std::io::Result<VecDeque<(u64, u64, Vec<u8>)>>;
}

#[async_trait]
pub trait AsyncByteRangeRead: ByteRangeMultiple {
    async fn read_to_vec(self) -> std::io::Result<VecDeque<(u64, u64, Vec<u8>)>>;
    async fn read_to_vec_range(
        self,
        start: u64,
        end: u64,
    ) -> std::io::Result<VecDeque<(u64, u64, Vec<u8>)>>;
}
