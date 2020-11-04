use crate::hal::port::mode::{Floating, Input, PullUp, TriState};
use crate::hal::port::portb::{PB0, PB2, PB3, PB4, PB5};
use crate::hal::port::portc::{PC0, PC1, PC2, PC3};
use crate::hal::port::portd::{PD6, PD7};
use crate::hal::port::porte::{PE0, PE1, PE2};

use crate::hal::port::Pin;
use crate::hal::prelude::*;
use crate::DDR;
use ufmt::derive::uDebug;

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
        // Find the column that is connected to the rows (which one we will determine next)
        let col = self.cols.iter().position(|c| c.is_low().void_unwrap());

        if let Some(col) = col {
            // Disconnect pins from the matrix
            for pin in self.rows.iter_mut() {
                pin.set_high().void_unwrap();
            }

            let mut row = None;
            for (idx, pin) in self.rows.iter_mut().enumerate() {
                // Locate the row that is connected to the column by connecting them (driving the pin low)
                pin.set_low().void_unwrap();
                if self.cols[col].is_low().void_unwrap() {
                    row = Some(idx);
                    break;
                }
                pin.set_high().void_unwrap();
            }

            // Set all pins low again to be able to start over again
            for rpin in self.rows.iter_mut() {
                rpin.set_low().void_unwrap();
            }

            if let Some(row) = row {
                return Some((col as u8) * 7 + (row as u8));
            }
        }
        None
    }
}

#[repr(u8)]
#[derive(uDebug)]
pub enum Key {
    Power,
    Dvd,
    Sat,
    Tv,
    Mute,
    TvAv,
    Plus,
    Minus,
    Pns,
    FastForward,
    FastBackward,
    Next,
    Prev,

    ProgramUp,
    ProgramDown,

    VolumeUp,
    VolumeDown,
    LeftRight,

    NavigationUp,
    NavigationLeft,
    NavigationRight,
    NavigationDown,

    Menu,
    Ok,
    Exit,
    Cancel,
    Schedule,
    Info,
    Zoom,

    Blue,
    Yellow,
    Green,
    Red,

    Three,
    Two,
    One,
    Six,
    Five,
    Four,
    Nine,
    Eight,
    Seven,
    Repeat,
    Zero,
    Plus10,

    Invalid,
}

impl From<u8> for Key {
    fn from(keyval: u8) -> Self {
        use Key::*;
        match keyval {
            0 => Power,
            1 => Dvd,
            2 => Sat,
            3 => Tv,
            4 => Mute,
            5 => TvAv,
            6 => Plus,
            7 => Minus,
            8 => Pns,
            9 => FastForward,
            10 => FastBackward,
            12 => Next,
            15 => Prev,

            11 => ProgramUp,
            14 => ProgramDown,

            13 => VolumeUp,
            16 => VolumeDown,
            17 => LeftRight,

            18 => NavigationUp,
            23 => NavigationLeft,
            24 => NavigationRight,
            26 => NavigationDown,

            20 => Menu,
            22 => Ok,
            28 => Exit,
            29 => Cancel,
            30 => Schedule,
            31 => Info,
            32 => Zoom,

            33 => Blue,
            34 => Yellow,
            35 => Green,
            36 => Red,

            37 => Three,
            38 => Two,
            39 => One,
            40 => Six,
            41 => Five,
            42 => Four,
            43 => Nine,
            44 => Eight,
            45 => Seven,
            46 => Repeat,
            47 => Zero,
            48 => Plus10,

            _ => Key::Invalid,
        }
    }
}
