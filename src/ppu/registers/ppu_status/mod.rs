use std::ops::{Deref, DerefMut};

#[cfg(test)]
mod spec_tests;

const FLG_VBLANK: u8 = 0b10000000;
const FLG_SPRITE_ZERO_HIT: u8 = 0b01000000;
const FLG_SPRITE_OVERFLOW: u8 = 0b00100000;

#[derive(Copy, Clone)]
pub struct PpuStatus {
    reg: u8,
}

impl Deref for PpuStatus {
    type Target = u8;

    fn deref(&self) -> &u8 {
        &self.reg
    }
}

impl DerefMut for PpuStatus {
    fn deref_mut(&mut self) -> &mut u8 {
        &mut self.reg
    }
}

impl PpuStatus {
    pub fn new() -> Self {
        PpuStatus { reg: 0 }
    }

    pub fn in_vblank(self) -> bool {
        *self & FLG_VBLANK != 0
    }

    pub fn sprite_zero_hit(self) -> bool {
        *self & FLG_SPRITE_ZERO_HIT != 0
    }

    pub fn sprite_overflow(self) -> bool {
        *self & FLG_SPRITE_OVERFLOW != 0
    }
}