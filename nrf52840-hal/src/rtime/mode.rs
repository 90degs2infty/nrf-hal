//! Hal-level interface to a timer's timer mode.
//!
//! See [Nordic's docs](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29) for details.

use core::marker::PhantomData;

/// Type indicating a timer running in low power counter mode.
///
/// See Nordic's documentation on the
/// [`MODE` register](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29_4_10#register.MODE)
/// for details.
pub struct Counter;

/// Type indicating a timer running in timer mode.
///
/// See Nordic's documentation on the
/// [`MODE` register](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29_4_10#register.MODE)
/// for details.
pub struct Timer<P> {
    prescaler: PhantomData<P>,
}
