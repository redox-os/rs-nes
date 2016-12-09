#[macro_use]
extern crate log;
extern crate env_logger;

extern crate rs_nes;

use rs_nes::cpu::*;
use rs_nes::cpu::debugger::*;
use rs_nes::memory::Memory;
use rs_nes::memory::nes_memory::NesMemory;
use rs_nes::rom::NesRom;
use rs_nes::disassembler::*;

const PC_START: u16 = 0x8000;

fn main() {
    env_logger::init().unwrap();
    let rom = NesRom::read("test_roms/mario.nes").unwrap();
    let mem = NesMemory::new(rom);

    let mut buf = Vec::new();
    mem.dump(&mut buf);
    let instructions = InstructionDecoder::new(&buf, PC_START as usize).collect::<Vec<Instruction>>();
    let mut debugger = http_debugger::HttpDebugger::new(instructions);
    debugger.start().unwrap();
    let mut cpu = Cpu::new(mem, debugger);
    cpu.registers.pc = PC_START;
    loop {
        cpu.step();
    }
}