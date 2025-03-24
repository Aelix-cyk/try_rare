use crate::dram::*;
use crate::exception::*;
use crate::parameter::*;

pub struct Bus {
    dram: Dram,
}

impl Bus {
    pub fn new(mem: Vec<u8>) -> Self {
        Self {
            dram: Dram::new(mem),
        }
    }

    pub fn load(&self, addr: usize, size: usize) -> Result<u64, Exception> {
        match addr {
            DRAM_BASE..=DRAM_END => self.dram.load(addr - DRAM_BASE, size),
            _ => Err(Exception::LoadAccessFault(addr as u64)),
        }
    }

    pub fn store(&mut self, addr: usize, size: usize, data: u64) -> Result<(), Exception> {
        match addr {
            DRAM_BASE..=DRAM_END => self.dram.store(addr - DRAM_BASE, size, data),
            _ => Err(Exception::StoreAMOAccessFault(addr as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn byte_ls() {
        let mut bus = Bus::new(vec![0x0, 0x1, 0x2, 0x3]);
        assert_eq!(bus.load(DRAM_BASE + 0, 8), Ok(0x0));
        assert_eq!(bus.load(DRAM_BASE + 1, 8), Ok(0x1));
        assert_eq!(bus.load(DRAM_BASE + 2, 8), Ok(0x2));
        assert_eq!(bus.load(DRAM_BASE + 3, 8), Ok(0x3));
        bus.store(DRAM_BASE + 0, 8, 3).unwrap();
        bus.store(DRAM_BASE + 1, 8, 2).unwrap();
        bus.store(DRAM_BASE + 2, 8, 1).unwrap();
        bus.store(DRAM_BASE + 3, 8, 0).unwrap();
        assert_eq!(bus.load(DRAM_BASE + 0, 8), Ok(0x3));
        assert_eq!(bus.load(DRAM_BASE + 1, 8), Ok(0x2));
        assert_eq!(bus.load(DRAM_BASE + 2, 8), Ok(0x1));
        assert_eq!(bus.load(DRAM_BASE + 3, 8), Ok(0x0));
    }
}
