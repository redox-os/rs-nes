use cpu::Cpu;
use cpu::opcodes::addressing::AddressingMode;
use memory::Memory;

pub struct Implied;

impl<M: Memory> AddressingMode<M> for Implied {
    type Output = ();

    fn read(&self) -> Self::Output {
        ()
    }

    fn write<F: Fn(&Cpu<M>)>(&self, _: &mut Cpu<M>, _: u8, _: F) {
        unimplemented!()
    }
}