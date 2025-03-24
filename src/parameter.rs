pub const DRAM_BASE: usize = 0x8000_0000;
pub const DRAM_SIZE: usize = 128 * (1 << 20);
pub const DRAM_END: usize = DRAM_BASE + DRAM_SIZE - 1;
