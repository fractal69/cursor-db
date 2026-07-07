use std::mem::size_of;

pub const MAGIC: u32 = 0x4D_44_54_4B; // MDTK (Market Data TicKs)
pub const VERSION: u32 = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub magic: u32,
    pub version: u32,
    pub trade_count: u64,
}

#[repr(C, align(8))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Trade {
    pub id: u64,
    pub time: u64,
    pub price: u64,
    pub qty: u64,
    pub is_buyer_maker: u8,
    pub _padding: [u8; 7],
}

pub const HEADER_SIZE: usize = size_of::<Header>();
pub const TRADE_SIZE: usize = size_of::<Trade>();

//----------------------------------------------------------------------------------------------------------------------
// TESTS
//----------------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::{align_of, size_of};

    #[test]
    fn layout() {
        assert_eq!(HEADER_SIZE, 16);
        assert_eq!(TRADE_SIZE, 40);
    }

    #[test]
    fn binary_layout_is_stable() {
        // Header
        assert_eq!(size_of::<Header>(), 16);
        assert_eq!(align_of::<Header>(), 8);

        // Trade
        assert_eq!(size_of::<Trade>(), 40);
        assert_eq!(align_of::<Trade>(), 8);

        // El archivo binario depende de estos tamaños.
        assert_eq!(HEADER_SIZE, 16);
        assert_eq!(TRADE_SIZE, 40);
    }
}

