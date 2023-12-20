#![no_std]

// #[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

pub use atsamd_hal as hal;
pub use hal::*;

use hal::clock::GenericClockController;
use hal::sercom::{
    uart::{self, BaudMode, Oversampling},
    IoSet3, IoSet4,
};
use hal::time::Hertz;

hal::bsp_peripherals!(SERCOM2 { UartSercom2 }, SERCOM1 { UartSercom1 });

hal::bsp_pins!(
    PA00 {
        /// Pin 1, UART tx, P14
        name: tx2,
        aliases: {
            AlternateD: UartTx1
        }
    }
    PA01 {
        /// Pin 2, UART rx, P13
        name: rx2,
        aliases: {
            AlternateD: UartRx1
        }
    }
    PA09 {
        /// Pin 18, UART tx
        name: tx1,
        aliases: {
            AlternateD: UartTx2
        }
    }
    PA08 {
        /// Pin 17, UART rx
        name: rx1,
        aliases: {
            AlternateD: UartRx2
        }
    }
    PB31 {
        /// Pin 59, LED1
        name: led1,
    }
    PB30 {
        /// Pin 60, LED2
        name: led2,
    }
);

// UART 1
/// UART pads for the labelled RX & TX pins
pub type UartPads2 = uart::Pads<UartSercom2, IoSet3, UartRx2, UartTx2>;
/// UART device for the labelled RX & TX pins
pub type Uart2 = uart::Uart<uart::Config<UartPads2>, uart::Duplex>;

// UART 2
/// UART pads for the labelled RX & TX pins
pub type UartPads1 = uart::Pads<UartSercom1, IoSet4, UartRx1, UartTx1>;
/// UART device for the labelled RX & TX pins
pub type Uart1 = uart::Uart<uart::Config<UartPads1>, uart::Duplex>;

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart1(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: UartSercom1,
    mclk: &mut pac::MCLK,
    rx: impl Into<UartRx1>,
    tx: impl Into<UartTx1>,
) -> Uart1 {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom1_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(rx.into()).tx(tx.into());
    uart::Config::new(mclk, sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart2(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: UartSercom2,
    mclk: &mut pac::MCLK,
    rx: impl Into<UartRx2>,
    tx: impl Into<UartTx2>,
) -> Uart2 {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom1_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(rx.into()).tx(tx.into());
    uart::Config::new(mclk, sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}
