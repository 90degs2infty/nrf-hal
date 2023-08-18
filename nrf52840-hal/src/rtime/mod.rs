//! A hal-level interface to the timer peripheral.
//!
//! Note that this module kind of by-passes [`nrf52480_hal`'s `timer` module](https://docs.rs/nrf52840-hal/latest/nrf52840_hal/timer/index.html)
//!
//! See [Nordic's docs](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29) for a general overview of the underlying hardware.

pub mod bitmode;
pub mod interrupt;
pub mod mode;
pub mod prescaler;
pub mod state;
pub mod timer;
