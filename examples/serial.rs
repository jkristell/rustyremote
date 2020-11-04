#![no_std]
#![no_main]

use board::prelude::*;
use board_kontrolir as board;

use avr_hal_generic::hal::digital::v2::ToggleableOutputPin;
use board_kontrolir::keypad::{Key, Keypad};
use panic_halt as _;

#[board::entry]
fn main() -> ! {
    let dp = board::Peripherals::take().unwrap();

    let mut pins = board::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);

    let mut serial = board::Serial::new(
        dp.USART0,
        pins.rxd,
        pins.txd.into_output(&mut pins.ddr),
        57600,
    );

    let mut led = pins.led.into_output(&mut pins.ddr);

    let mut kbd = Keypad::new(
        pins.row1,
        pins.row2,
        pins.row3,
        pins.row4,
        pins.row5,
        pins.row6,
        pins.row7,
        pins.col1,
        pins.col2,
        pins.col3,
        pins.col4,
        pins.col5,
        pins.col6,
        pins.col7,
        &mut pins.ddr,
    );

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    loop {
        board::delay_ms(100);

        if let Some(code) = kbd.scancode() {
            ufmt::uwriteln!(&mut serial, "Scancode {:?} ({}) \r", Key::from(code), code)
                .void_unwrap();
            led.toggle().void_unwrap();
        }
    }
}
