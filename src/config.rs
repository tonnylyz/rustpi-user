pub const RECURSIVE_PAGE_TABLE_TOP: usize = 0x80_0000_0000;
pub const RECURSIVE_PAGE_TABLE_BTM: usize = 0x7f_c000_0000;
pub const EXCEPTION_STACK_TOP: usize = 0x7f_c000_0000;
pub const EXCEPTION_STACK_BTM: usize = 0x7f_bfff_f000;
pub const TRAVERSE_LIMIT: usize = 0x7f_8000_0000;
pub const STACK_TOP: usize = 0x7f_8000_0000;
pub const STACK_BTM: usize = 0x7f_7fff_f000;

pub const PAGE_SIZE: usize = 4096;

pub const PAGE_TABLE_L1_SHIFT: usize = 30;
pub const PAGE_TABLE_L2_SHIFT: usize = 21;
pub const PAGE_TABLE_L3_SHIFT: usize = 12;

pub const WORD_SHIFT: usize = 3;
pub const WORD_SIZE: usize = 8;
