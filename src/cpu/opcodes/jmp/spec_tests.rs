use super::Jmp;
use cpu::*;
use cpu::opcodes::OpCode;

#[test]
fn test() {
    let mut cpu = TestCpu::new_test();
    Jmp::execute_cycles(&mut cpu, 0xbeef_u16);
    assert_eq!(0xbeef, cpu.registers.pc);
}