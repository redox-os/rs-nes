use super::{ADDRESSABLE_MEMORY, Memory};
use errors::*;

#[cfg(feature = "debugger")]
use seahash;
use std::io::Write;

pub struct SimpleMemory {
    addr: [u8; ADDRESSABLE_MEMORY],
}

impl SimpleMemory {
    pub fn new() -> Self {
        SimpleMemory { addr: [0; ADDRESSABLE_MEMORY] }
    }

    pub fn store_many(&mut self, addr: u16, data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            self.store(addr + i as u16, *byte).unwrap();
        }
    }
}

impl Clone for SimpleMemory {
    fn clone(&self) -> Self {
        SimpleMemory { addr: self.addr }
    }
}

impl Default for SimpleMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory for SimpleMemory {
    fn store(&mut self, addr: u16, data: u8) -> Result<()> {
        let addr = addr as usize;
        self.addr[addr] = data;
        Ok(())
    }

    fn load(&self, addr: u16) -> Result<u8> {
        let addr = addr as usize;
        Ok(self.addr[addr])
    }

    fn dump<T: Write>(&self, writer: &mut T) {
        writer.write_all(&self.addr).unwrap();
    }

    #[cfg(feature = "debugger")]
    fn hash(&self) -> u64 {
        seahash::hash(&self.addr)
    }
}