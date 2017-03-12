#[macro_use]
extern crate env_logger;

extern crate glium;
extern crate log;
extern crate rs_nes;

use glium::{DisplayBuild, Surface};
use glium::glutin;
use rs_nes::cpu::*;
use rs_nes::memory::nes_memory::NesMemoryImpl;
use rs_nes::ppu::{Ppu, PpuImpl};
use rs_nes::rom::NesRom;
use rs_nes::screen::NesScreen;
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant};

static SCREEN_DIMENSIONS: (u32, u32) = (256, 240);

fn main() {
    env_logger::init().unwrap();

    // INIT NES
    let rom = NesRom::read("test_roms/mario.nes").unwrap();
    println!("ROM Mapper: {} CHR banks: {} CHR size: {}",
             rom.mapper,
             rom.chr_rom_banks,
             rom.chr.len());

    let screen = Rc::new(RefCell::new(NesScreen::default()));
    let ppu = PpuImpl::new(rom.clone(), screen.clone());
    let mem = NesMemoryImpl::new(rom, ppu);
    let mut cpu = Cpu::new(mem);
    cpu.reset().unwrap();

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_vsync()
        .build_glium()
        .unwrap();

    start_loop(cpu, || {
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => {
                    println!("WINDOW CLOSED!");
                    return Action::Stop;
                }
                _ => {
                    update_screen(&display, &screen);
                    return Action::Continue;
                }
            }
        }
        Action::Continue
    });
}

enum Action {
    Stop,
    Continue,
}

fn start_loop<F>(mut cpu: Cpu<NesMemoryImpl>, mut callback: F)
    where F: FnMut() -> Action
{
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => (),
        };

        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_time_stamp = Duration::new(0, 16666667);
        let mut show_frame = false;
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;

            loop {
                if cpu.step().unwrap() == Interrupt::Nmi || show_frame {
                    break;
                }
            }
            show_frame = true;
        }

        thread::sleep(fixed_time_stamp - accumulator);
    }
}

fn update_screen(display: &glium::Display, screen: &Rc<RefCell<NesScreen>>) {
    let target = display.draw();
    let borrowed_scr: NesScreen = screen.borrow().to_owned();
    let mut buf = vec![0_u8; 256 * 240 * 3];
    buf.clone_from_slice(&borrowed_scr.screen_buffer[..]);
    let screen = glium::texture::RawImage2d::from_raw_rgb_reversed(buf, SCREEN_DIMENSIONS);
    let screen_buffer = glium::Texture2d::new(display, screen).unwrap();
    screen_buffer.as_surface()
        .fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);
    target.finish().unwrap();
}
