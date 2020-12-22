#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use board_kontrolir as board;
use board::prelude::*;

use board::keypad::{Key, Keypad};
use board::timer::Timer1;

use panic_halt as _;
use core::mem;
use board_kontrolir::hal::port;
use board_kontrolir::timer::calc_constants;

static mut LED: mem::MaybeUninit<port::porte::PE3<port::mode::Output>> = mem::MaybeUninit::uninit();

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
    led.toggle().void_unwrap();
    unsafe {
        // SAFETY: Interrupts are not enabled at this point so we can safely write the global
        // variable here.  A memory barrier afterwards ensures the compiler won't reorder this
        // after any operation that enables interrupts.
        LED = mem::MaybeUninit::new(led);
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }

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


    const pre: (u8, u16) = calc_constants(1);
    ufmt::uwriteln!(&mut serial, "Pre: {:?}\r", pre).void_unwrap();

    //let mut t1 = Timer1::from_precalc(dp.TC1, pre.0, pre.1);
    let mut t1 = Timer1::new(dp.TC1,  4);
    t1.listen();


    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);

    // Enable interrupts globally
    unsafe {
        // SAFETY: Not inside a critical section and any non-atomic operations have been completed
        // at this point.
        avr_device::interrupt::enable();
    }


    loop {
        board::delay_ms(100);

        if let Some(code) = kbd.scancode() {
            ufmt::uwriteln!(
                &mut serial,
                "Scancode {:?} ({})\r",
                Key::from(code), code
            )
            .void_unwrap();
        }
    }
}


#[avr_device::interrupt(atmega328pb)]
fn TIMER1_COMPA() {
    let led = unsafe {
        // SAFETY: We _know_ that interrupts will only be enabled after the LED global was
        // initialized so this ISR will never run when LED is uninitialized.
        &mut *LED.as_mut_ptr()
    };

    led.toggle().void_unwrap();
}

