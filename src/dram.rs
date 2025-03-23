use crate::exception::*;
use crate::parameter::*;

pub struct Dram {
    dram: Vec<u8>,
}

impl Dram {
    pub fn new(mem: Vec<u8>) -> Self {
        let mut dram = vec![0; DRAM_SIZE as usize];
        dram.splice(..mem.len(), mem.into_iter());
        Self { dram }
    }

    pub fn load(&self, addr: usize, size: usize) -> Result<u64, Exception> {
        macro_rules! load_nbytes {
            ($size: literal) => {
                self.dram
                    .get(addr..addr + $size / 8)
                    .and_then(|bytes| {
                        <[u8; 8]>::try_from([bytes, &[0; 8 - $size / 8]].concat()).ok()
                    })
                    .map(|bytes| u64::from_le_bytes(bytes.try_into().unwrap()))
                    .ok_or(Exception::LoadAccessFault(addr as u64))
            };
        }
        match size {
            8 => load_nbytes!(8),
            16 => load_nbytes!(16),
            32 => load_nbytes!(32),
            64 => load_nbytes!(64),
            _ => Err(Exception::LoadAccessFault(addr as u64)),
        }
    }

    pub fn store(&mut self, addr: usize, size: usize, data: u64) -> Result<(), Exception> {
        match size {
            8 | 16 | 32 | 64 => {
                self.dram
                    .splice(addr..addr + size / 8, data.to_le_bytes().into_iter());
                Ok(())
            }
            _ => Err(Exception::StoreAMOAccessFault(addr as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn byte_ls() {
        let mut dram = Dram::new(vec![0x0, 0x1, 0x2, 0x3]);
        assert_eq!(dram.load(0, 8), Ok(0x0));
        assert_eq!(dram.load(1, 8), Ok(0x1));
        assert_eq!(dram.load(2, 8), Ok(0x2));
        assert_eq!(dram.load(3, 8), Ok(0x3));
        dram.store(0, 8, 3).unwrap();
        dram.store(1, 8, 2).unwrap();
        dram.store(2, 8, 1).unwrap();
        dram.store(3, 8, 0).unwrap();
        assert_eq!(dram.load(0, 8), Ok(0x3));
        assert_eq!(dram.load(1, 8), Ok(0x2));
        assert_eq!(dram.load(2, 8), Ok(0x1));
        assert_eq!(dram.load(3, 8), Ok(0x0));
    }
}
