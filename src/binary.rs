use crate::{
    cursor::Cursor,
    trade::{HEADER_SIZE, Header, MAGIC, TRADE_SIZE, Trade, VERSION},
};
use anyhow::{Result, bail};
use memmap2::Mmap;
use std::{fs::File, path::Path, slice};

pub struct BinaryFile {
    mmap: Mmap,
    header: Header,
    trades: *const [Trade],
}

unsafe impl Send for BinaryFile {}

unsafe impl Sync for BinaryFile {}

impl BinaryFile {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        assert_eq!(HEADER_SIZE % std::mem::align_of::<Trade>(), 0);

        let file = File::open(path)?;

        let mmap = unsafe { Mmap::map(&file)? };

        if mmap.len() < HEADER_SIZE {
            bail!("File too small");
        }

        let header = Header {
            magic: u32::from_le_bytes(mmap[0..4].try_into()?),
            version: u32::from_le_bytes(mmap[4..8].try_into()?),
            trade_count: u64::from_le_bytes(mmap[8..16].try_into()?),
        };

        if header.magic != MAGIC {
            bail!("Invalid magic");
        }

        if header.version != VERSION {
            bail!("Unsupported version");
        }

        let expected_size = HEADER_SIZE + header.trade_count as usize * TRADE_SIZE;

        if mmap.len() != expected_size {
            bail!(
                "Corrupted file. Expected {} bytes, got {}",
                expected_size,
                mmap.len()
            );
        }

        let trades = unsafe {
            slice::from_raw_parts(
                mmap.as_ptr().add(HEADER_SIZE) as *const Trade,
                header.trade_count as usize,
            )
        };

        Ok(Self {
            mmap,
            header,
            trades,
        })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.header.trade_count as usize
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.header.trade_count == 0
    }

    #[inline]
    pub fn header(&self) -> &Header {
        &self.header
    }

    #[inline]
    pub fn trade(&self, index: usize) -> Option<&Trade> {
        unsafe { (&*self.trades).get(index) }
    }

    #[inline]
    pub fn trades(&self) -> &[Trade] {
        unsafe { &*self.trades }
    }

    #[inline]
    pub fn cursor(&self) -> Cursor<'_> {
        Cursor::new(self.trades())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary::BinaryFile;

    #[test]
    fn trade_count_matches_file_size() {
        use std::fs;

        let path = "./output/ticks.bin";

        let metadata = fs::metadata(path).unwrap();

        let file = BinaryFile::open(path).unwrap();

        let expected = HEADER_SIZE as u64 + file.len() as u64 * TRADE_SIZE as u64;

        assert_eq!(metadata.len(), expected);
    }
}
