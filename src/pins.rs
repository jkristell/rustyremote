use hal::port::PortExt;

crate::avr_hal_generic::impl_board_pins! {
    #[port_defs]
    use hal::port;

    /// Generic DDR that works for all ports
    pub struct DDR {
        portb: crate::pac::PORTB,
        portc: crate::pac::PORTC,
        portd: crate::pac::PORTD,
        porte: crate::pac::PORTE,
    }

    /// Reexport of the pins with the name from the pin map table
    pub struct Pins {
        pub irin1: portd::pd2::PD2,
        pub irin2: portd::pd3::PD3,
        /// Infrared out - Timer1
        pub irout: portb::pb1::PB1,

        /// Front panel led
        pub led: porte::pe3::PE3,

        // Matrix
        pub row1: portb::pb0::PB0,
        pub row2: portb::pb2::PB2,
        pub row3: portb::pb3::PB3,
        pub row4: portb::pb4::PB4,
        pub row5: portb::pb5::PB5,
        pub row6: portd::pd6::PD6,
        pub row7: portd::pd7::PD7,

        pub col1: portc::pc0::PC0,
        pub col2: portc::pc1::PC1,
        pub col3: portc::pc2::PC2,
        pub col4: portc::pc3::PC3,
        pub col5: porte::pe0::PE0,
        pub col6: porte::pe1::PE1,
        pub col7: porte::pe2::PE2,

        /// Serial
        pub rxd: portd::pd0::PD0,
        pub txd: portd::pd1::PD1,

        /// i2c eeprom?
        pub sclk: portc::pc4::PC4,
        pub sda: portc::pc5::PC5,
    }
}
