pub const DRAM_BASE: u64 = 0x8000_0000;
pub const DRAM_SIZE: u64 = 128 * (1 << 20);
pub const DRAM_END: u64 = DRAM_BASE + DRAM_SIZE - 1;
