//let pd2: PD2<mode::Input<mode::PullUp>> = pd2.into_pull_up_input(&mut portd.ddr);

use crate::hal::port::portb::{PB0, PB2, PB3, PB4, PB5};
use crate::hal::port::mode::{Input, PullUp, Output, Floating};
use crate::hal::port::portd::{PD6, PD7};
use crate::hal::port::portc::{PC0, PC1, PC2, PC3};
use crate::hal::port::porte::PE0;

use crate::hal::prelude::*;
use crate::DDR;

pub struct KeyboardMatrix {
    pub r1: PB0<Output>,
    pub r2: PB2<Output>,
    pub r3: PB3<Output>,
    pub r4: PB4<Output>,
    pub r5: PB5<Output>,
    pub r6: PD6<Output>,
    pub r7: PD7<Output>,

    pub c1: PC0<Input<PullUp>>,
    pub c2: PC1<Input<PullUp>>,
    pub c3: PC2<Input<PullUp>>,
    pub c4: PC3<Input<PullUp>>,
    pub c5: PE0<Input<PullUp>>,
}

impl KeyboardMatrix {

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
       ddr: &mut DDR,
    ) -> Self {
        let mut s = Self {
            r1: r1.into_output(ddr),
            r2: r2.into_output(ddr),
            r3: r3.into_output(ddr),
            r4: r4.into_output(ddr),
            r5: r5.into_output(ddr),
            r6: r6.into_output(ddr),
            r7: r7.into_output(ddr),
            c1: c1.into_pull_up_input(ddr),
            c2: c2.into_pull_up_input(ddr),
            c3: c3.into_pull_up_input(ddr),
            c4: c4.into_pull_up_input(ddr),
            c5: c5.into_pull_up_input(ddr)
        };

        s.
    }

    pub fn key(&self) -> (u8, u8) {



        let row = 0;
            //u8::from(self.row1.is_low().void_unwrap()) << 0 |
            //u8::from(self.row2.is_low().void_unwrap()) << 1 |
            //u8::from(self.row3.is_low().void_unwrap()) << 2 |
            //u8::from(self.row4.is_low().void_unwrap()) << 3 |
            //u8::from(self.row5.is_low().void_unwrap()) << 4 |
            //u8::from(self.row6.is_low().void_unwrap()) << 5 |
            //u8::from(self.row7.is_low().void_unwrap()) << 6;
        let col =
            u8::from(self.col1.is_low().void_unwrap()) << 0 |
                u8::from(self.col2.is_low().void_unwrap()) << 1 |
                u8::from(self.col3.is_low().void_unwrap()) << 2 |
                u8::from(self.col4.is_low().void_unwrap()) << 3 |
                u8::from(self.col5.is_low().void_unwrap()) << 4;


        (row, col)
    }

}

