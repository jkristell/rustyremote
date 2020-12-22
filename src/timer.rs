// Timer Configuration:
// - WGM = 4: CTC mode (Clear Timer on Compare Match)
// - Prescaler 256
// - OCR1A = 15624
//
// => F = 16 MHz / (256 * (1 + 15624)) = 4 Hz
//     (^ this formula I deduced from reading the datasheet)
//
// => The LED will toggle at 4 Hz, thus it blinks at 2 Hz

use crate::hal::pac::TC1;
use core::convert::{TryInto, TryFrom};

pub struct Timer1 {
    periph: TC1,
}

/// Calculate prescaler and TOP value
/// This should be evaluated at compile time
pub const fn calc_constants(freq: u32) -> (u8, u16) {

    let clk_io = 16_000_000u32;

    let mut presval = [1, 8, 64, 256, 1024];
    let mut index = 0;

    loop {
        let prescaler = presval[index];

        if let Some(val) = clk_io.checked_div(freq * prescaler) {
            if val < (u16::MAX as u32) {
                return (index as u8 + 1, val as u16)
            }
        }

        index += 1;
        if index > presval.len() {
            break;
        }
    }

    (0, 0)
}


impl Timer1 {

    pub fn new(tc1: TC1, freq: u32) -> Self {

        let CC = calc_constants(freq);

        Self::from_precalc(tc1, CC.0, CC.1)
    }

    pub fn from_precalc(tc1: TC1, prescaler: u8, top: u16) -> Self {
        tc1.tccr1a.write(|w| w.wgm1().bits(0b00));
        tc1.tccr1b.write(|w| w.cs1().bits(prescaler).wgm1().bits(0b01));
        tc1.ocr1a.write(|w| unsafe { w.bits(top) });

        Timer1 {
            periph: tc1,
        }

    }

    pub fn listen(&mut self) {
        // Enable the timer interrupt
        self.periph.timsk1.write(|w| w.ocie1a().set_bit());
    }

    pub fn debug(&self) -> (u8, u8, u16) {
        (self.periph.tccr1a.read().bits(),
         self.periph.tccr1b.read().bits(),
         self.periph.ocr1a.read().bits())
    }
    pub fn flags(&self) -> u8 {
        (self.periph.tifr1.read().bits())
    }
}


