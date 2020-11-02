#![no_std]
#![no_main]

use board_kontrolir as board;
use board::prelude::*;

use panic_halt as _;
use board_kontrolir::keyboard::KeyboardMatrix;

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


    /*
    let kbd = KeyboardMatrix {
        row1: pins.row1.into_output(&mut pins.ddr),
        row2: pins.row2.into_output(&mut pins.ddr),
        row3: pins.row3.into_output(&mut pins.ddr),
        row4: pins.row4.into_output(&mut pins.ddr),
        row5: pins.row5.into_output(&mut pins.ddr),
        row6: pins.row6.into_output(&mut pins.ddr),
        row7: pins.row7.into_output(&mut pins.ddr),

        col1: pins.col1.into_pull_up_input(&mut pins.ddr),
        col2: pins.col2.into_pull_up_input(&mut pins.ddr),
        col3: pins.col3.into_pull_up_input(&mut pins.ddr),
        col4: pins.col4.into_pull_up_input(&mut pins.ddr),
        col5: pins.col5.into_pull_up_input(&mut pins.ddr),
    };
     */

    let kbd = KeyboardMatrix::new(
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
&mut pins.ddr,
    };


    ufmt::uwriteln!(
        &mut serial,
         "Hello from Arduino!\r"
    ).void_unwrap();

    loop {
        // Read a byte from the serial connection
        //let b = nb::block!(serial.read()).void_unwrap();

        board::delay_ms(1000);
        let (row, col) = kbd.key();

        // Answer
        ufmt::uwriteln!(&mut serial, "Got {}, {}!\r", row, col).void_unwrap();
    }
}
