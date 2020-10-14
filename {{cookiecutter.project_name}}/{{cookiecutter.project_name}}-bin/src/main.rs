#![no_std]
#![no_main]

// Pull in the panic handler from panic-halt
extern crate panic_halt;

use arduino_uno::prelude::*;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        9600,
    );

    ufmt::uwriteln!(&mut serial, "Starting main loop...\r").void_unwrap();
    loop {
        ufmt::uwriteln!(&mut serial, "main loop...\r").void_unwrap();
        arduino_uno::delay_ms(1000);
    }
}

