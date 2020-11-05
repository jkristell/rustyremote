# Board Support for the KontroLIR Remote control

This is a WIP crate for supporting the [AnalysIR] remote control [KontroLIR]. You can find the remote and other great products at their [shop][AnalysIrShop]!

The plan is to use the [Infrared] crate to enable fun Rust projects with a real remote control.

Uses the [avr-device][AvrDevice] and [avr-hal][AvrHal] crates by [Rahix] and others and [Infrared] by me.

## Status
 - [x] Support for the mcu in avr-device [Pull request](https://github.com/Rahix/avr-device/pull/55)
 - [x] Support for the mcu in avr-hal [Pull request](https://github.com/Rahix/avr-hal/pull/96)
 - [ ] Figure out how to use this as an "external" BSP
 - [x] Figure out the pin mappings
 - [x] Read keypad
 - [x] Blink Led
 - [x] Serial to host
 - [ ] Clocks and timers
 - [ ] Sending IR
 - [ ] Receiving IR
 - [ ] Eeprom
 - [ ] Keypad Interrupts
 - [ ] Low power states

## Examples
 - `serial.rs` Read Keypad and get the pressed button 

## Howto

- Remove batteries before connecting USB.
- `cargo run --release --example serial`
- `picocom -b 57600 /dev/ttyUSB0`

## Hardware
![Picture of the KontroLIR Remote control](https://wiki.analysir.com/images/thumb/0/0d/KontroLIR_photo1.jpg/255px-KontroLIR_photo1.jpg "KontroLIR")

 - Custom board from [AnalysIr] in a remote control shell
 - [Atmega328pb]
 
## Acknowledgements

The pin map is based on the code from [AnalysIr] with the following license:
```
Author: AnalysIR
Copyright 2019, AnalysIR
Website: https://www.AnalysIR.com/blog/
WiKi: https://wiki.analysir.com/index.php?title=KontroLIR_-_Arduino_compatible_IR_remote_control
Licence: MIT (permissive), see file KIR_Copyright.h
Support: https://IRforum.AnalysIR.com/
Contact: via contact form on website above. If in doubt about permitted use, contact us directly.
KontrolIR Hardware: can be purchased via our website (& in future official partners)

Version 1.0.0
Notice: This  header must be placed at the top of every file (Including derivatives/forks)
```


[AnalysIR]: https://www.analysir.com/blog/
[KontroLIR]: https://wiki.analysir.com/index.php?title=KontroLIR_-_Arduino_compatible_IR_remote_control
[AnalysIrShop]: https://www.analysir.com/blog/shop/
[Atmega328pb]: https://www.microchip.com/wwwproducts/en/atmega328pb
[AvrHal]: https://github.com/Rahix/avr-hal/
[AvrDevice]: https://github.com/Rahix/avr-device/
[Rahix]: https://github.com/Rahix/
[Infrared]: https://github.com/jkristell/infrared
