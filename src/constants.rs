pub const STACK_LOC: u16 = 0x100;
pub const IRQ_VECTOR_LOC: u16 = 0xfffe;

pub const FL_CARRY: u8 = 1 << 0;
pub const FL_ZERO: u8 = 1 << 1;
pub const FL_INTERRUPT_DISABLE: u8 = 1 << 2;
pub const FL_BRK: u8 = 1 << 4;
pub const FL_OVERFLOW: u8 = 1 << 6;
pub const FL_SIGN: u8 = 1 << 7;
