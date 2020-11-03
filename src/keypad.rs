//let pd2: PD2<mode::Input<mode::PullUp>> = pd2.into_pull_up_input(&mut portd.ddr);

use crate::hal::port::mode::{Floating, Input, Output, PullUp, TriState};
use crate::hal::port::portb::{PB0, PB2, PB3, PB4, PB5};
use crate::hal::port::portc::{PC0, PC1, PC2, PC3};
use crate::hal::port::portd::{PD6, PD7};
use crate::hal::port::porte::{PE0, PE1, PE2};

use crate::hal::port::Pin;
use crate::hal::prelude::*;
use crate::DDR;
use avr_hal_generic::hal::digital::v2::{InputPin, OutputPin};

pub struct Keypad {
    rows: [Pin<TriState>; 7],
    cols: [Pin<Input<PullUp>>; 7],
}

impl Keypad {
    pub fn new(
        r1: PB0<Input<Floating>>,
        r2: PB2<Input<Floating>>,
        r3: PB3<Input<Floating>>,
        r4: PB4<Input<Floating>>,
        r5: PB5<Input<Floating>>,
        r6: PD6<Input<Floating>>,
        r7: PD7<Input<Floating>>,

        c1: PC0<Input<Floating>>,
        c2: PC1<Input<Floating>>,
        c3: PC2<Input<Floating>>,
        c4: PC3<Input<Floating>>,
        c5: PE0<Input<Floating>>,
        c6: PE1<Input<Floating>>,
        c7: PE2<Input<Floating>>,
        ddr: &mut DDR,
    ) -> Self {
        let mut rows = [
            r1.into_tri_state(ddr).downgrade(),
            r2.into_tri_state(ddr).downgrade(),
            r3.into_tri_state(ddr).downgrade(),
            r4.into_tri_state(ddr).downgrade(),
            r5.into_tri_state(ddr).downgrade(),
            r6.into_tri_state(ddr).downgrade(),
            r7.into_tri_state(ddr).downgrade(),
        ];

        let cols = [
            c1.into_pull_up_input(ddr).downgrade(),
            c2.into_pull_up_input(ddr).downgrade(),
            c3.into_pull_up_input(ddr).downgrade(),
            c4.into_pull_up_input(ddr).downgrade(),
            c5.into_pull_up_input(ddr).downgrade(),
            c6.into_pull_up_input(ddr).downgrade(),
            c7.into_pull_up_input(ddr).downgrade(),
        ];

        // Drive the pins low
        for rpin in rows.iter_mut() {
            rpin.set_low().void_unwrap();
        }

        Self { rows, cols }
    }

    pub fn scancode(&mut self) -> Option<u8> {

        let colindex = self.cols.iter().position(|c| c.is_low().void_unwrap());

        if let Some(col) = colindex {
            for pin in self.rows.iter_mut() {
                pin.set_high().void_unwrap();
            }

            let mut row = None;
            for (idx, pin) in self.rows.iter_mut().enumerate() {
                pin.set_low().void_unwrap();
                if self.cols[col].is_low().void_unwrap() {
                    row = Some(idx);
                    break;
                }
                pin.set_high().void_unwrap();
            }

            // Set the pins low again
            for rpin in self.rows.iter_mut() {
                rpin.set_low().void_unwrap();
            }

            if let Some(row) = row {
                return Some((col as u8) * 7 + (row as u8) );
            }
        }
        None
    }
}
