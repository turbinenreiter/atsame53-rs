#![no_std]
#![no_main]

use attest as bsp;
#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::{entry, hal, periph_alias};
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let pins = bsp::Pins::new(peripherals.PORT);
    let mut led1 = pins.led1.into_push_pull_output();
    let mut led2 = pins.led2.into_push_pull_output();
    let uart_rx1 = pins.rx1;
    let uart_tx1 = pins.tx1;
    let uart_rx2 = pins.rx2;
    let uart_tx2 = pins.tx2;
    let mut delay = Delay::new(core.SYST, &mut clocks);

    let uart_sercom2 = periph_alias!(peripherals.uart_sercom2);
    let uart_sercom1 = periph_alias!(peripherals.uart_sercom1);

    // this causes the program to hang
    // let mut uart1 = bsp::uart2(
    //     &mut clocks,
    //     19200.Hz(),
    //     uart_sercom2,
    //     &mut peripherals.MCLK,
    //     uart_rx1,
    //     uart_tx1,
    // );

    let mut uart2 = bsp::uart1(
        &mut clocks,
        19200.Hz(),
        uart_sercom1,
        &mut peripherals.MCLK,
        uart_rx2,
        uart_tx2,
    );

    loop {
        for byte in b"Hello, world!\r\n" {
            nb::block!(uart2.write(*byte)).unwrap();
        }
        // for byte in b"Hello, world!\r\n" {
        //     nb::block!(uart1.write(*byte)).unwrap();
        // }
        delay.delay_ms(500u16);
        led1.set_high().unwrap();
        led2.set_low().unwrap();
        delay.delay_ms(500u16);
        led1.set_low().unwrap();
        led2.set_high().unwrap();
    }
}
