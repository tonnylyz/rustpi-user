pub const LIMIT: usize = 0x8000_0000;
const STACK_TOP: usize = 0x8000_0000;
pub const RECURSIVE_PAGE_TABLE_BASE: usize = 0x8000_0000;

pub const PAGE_SIZE: usize = 4096;

pub const PAGE_TABLE_L1_SHIFT: usize = 30;
pub const PAGE_TABLE_L2_SHIFT: usize = 21;
pub const PAGE_TABLE_L3_SHIFT: usize = 12;

pub const WORD_SHIFT: usize = 3;
pub const WORD_SIZE: usize = 8;
