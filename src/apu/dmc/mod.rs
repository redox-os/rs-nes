pub trait Dmc: Default {
    fn write_4010(&mut self, val: u8);
    fn write_4011(&mut self, val: u8);
    fn write_4012(&mut self, val: u8);
    fn write_4013(&mut self, val: u8);
    fn zero_length_counter(&mut self);
}

#[derive(Default)]
pub struct DmcImpl {}

impl Dmc for DmcImpl {
    fn write_4010(&mut self, _: u8) {}

    fn write_4011(&mut self, _: u8) {}

    fn write_4012(&mut self, _: u8) {}

    fn write_4013(&mut self, _: u8) {}

    fn zero_length_counter(&mut self) {}
}
