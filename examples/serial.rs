#![no_std]
#![no_main]

use board_kontrolir as board;
use board::prelude::*;

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

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).void_unwrap();

        // Answer
        ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();
    }
}
