use super::{
    bitmode::{Width, W32},
    interrupt::{
        BasicIRQState, BasicIRQsDisabled, Disabled, Enabled, ExtendedIRQState, ExtendedIRQsDisabled,
    },
    mode::{Counter as CounterMode, Timer as TimerMode},
    prescaler::{Prescaler, P0},
    state::{Started, Stopped},
};
use core::marker::PhantomData;
use core::ops::Deref;
use nrf52_proc_macros::enclose;
use nrf_hal_common::{
    pac::{timer0::RegisterBlock as BasicRegBlock, timer3::RegisterBlock as ExtendedRegBlock},
    timer::Instance,
};

#[macro_use]
mod basic_macros;

/// HAL-level interface to timer peripheral.
pub struct Timer<T, W, C, S, I> {
    timer: T,
    w: PhantomData<W>,
    c: PhantomData<C>,
    s: PhantomData<S>,
    i: PhantomData<I>,
}

macro_rules! initialize_peripheral {
    ( $timer:expr, $( $irq:literal ),+ ) => {
        stop_timer!($timer);
        ensure_width_32!($timer);
        disable_interrupts!($timer,
            $(
                $irq
            ),+
        );
        unpend_interrupts!($timer,
            $(
                $irq
            ),+
        );
    };
}

impl<T> Timer<T, W32, TimerMode<P0>, Stopped, BasicIRQsDisabled>
where
    T: Instance + Deref<Target = BasicRegBlock>,
{
    /// Turn a PAC-level timer peripheral into a HAL-level timer running in timer mode.
    pub fn timer(timer: T) -> Self {
        initialize_peripheral!(timer, 0, 1, 2, 3);
        set_mode!(timer, timer);
        ensure_prescale_0!(timer);
        timer!(timer)
    }
}

impl<T> Timer<T, W32, TimerMode<P0>, Stopped, ExtendedIRQsDisabled>
where
    T: Instance + Deref<Target = ExtendedRegBlock>,
{
    /// Turn a PAC-level timer peripheral into a HAL-level timer running in timer mode.
    pub fn timer(timer: T) -> Self {
        initialize_peripheral!(timer, 0, 1, 2, 3, 4, 5);
        set_mode!(timer, timer);
        ensure_prescale_0!(timer);
        timer!(timer)
    }
}

impl<T> Timer<T, W32, CounterMode, Stopped, BasicIRQsDisabled>
where
    T: Instance + Deref<Target = BasicRegBlock>,
{
    /// Turn a PAC-level timer peripheral into a HAL-level timer running in counter mode.
    pub fn counter(timer: T) -> Self {
        initialize_peripheral!(timer, 0, 1, 2, 3);
        set_mode!(timer, counter);
        timer!(timer)
    }
}

impl<T> Timer<T, W32, CounterMode, Stopped, ExtendedIRQsDisabled>
where
    T: Instance + Deref<Target = ExtendedRegBlock>,
{
    /// Turn a PAC-level timer peripheral into a HAL-level timer running in counter mode.
    pub fn counter(timer: T) -> Self {
        initialize_peripheral!(timer, 0, 1, 2, 3, 4, 5);
        set_mode!(timer, counter);
        timer!(timer)
    }
}

impl<T, W, I, C> Timer<T, W, C, Stopped, I>
where
    T: Instance,
{
    /// Set a timer's bit with.
    ///
    /// See `Width` for details.
    pub fn set_counterwidth<W2: Width>(self) -> Timer<T, W2, C, Stopped, I> {
        set_width!(self.timer, W2);
        timer!(self.timer)
    }

    /// Start a timer.
    pub fn start(self) -> Timer<T, W, C, Started, I> {
        self.timer
            .as_timer0()
            .tasks_start
            .write(|w| w.tasks_start().set_bit());
        timer!(self.timer)
    }
}

impl<T, W, I, P> Timer<T, W, TimerMode<P>, Stopped, I>
where
    T: Instance,
{
    /// Set a timer's prescale value.
    ///
    /// See `Prescaler` for details.
    pub fn set_prescale<P2: Prescaler>(self) -> Timer<T, W, TimerMode<P2>, Stopped, I> {
        self.timer
            .as_timer0()
            .prescaler
            .write(|w| unsafe { w.bits(P2::VAL) });
        timer!(self.timer)
    }
}

impl<T, W, I, C> Timer<T, W, C, Started, I>
where
    T: Instance,
{
    /// Stop a timer.
    pub fn stop(self) -> Timer<T, W, C, Stopped, I> {
        self.timer
            .as_timer0()
            .tasks_stop
            .write(|w| w.tasks_stop().set_bit());
        timer!(self.timer)
    }
}

impl<T, W, I> Timer<T, W, CounterMode, Started, I>
where
    T: Instance,
{
    /// Increase this counter's value by one.
    ///
    /// Note that increasing the counter's value may cause it to overflow, in
    /// which case the counter starts counting from zero again.
    pub fn tick(&mut self) {
        self.timer
            .as_timer0()
            .tasks_count
            .write(|w| w.tasks_count().set_bit());
    }
}

macro_rules! define_cc {
    ( $reg:ty, $istate:ty, $num:literal, $( $ilane:ty ),* | $ilane_prime:ty ) => {
        paste::paste! {

            impl<T, S, W, $( $ilane ),* , C> Timer<T, W, C, S, enclose!(Enabled at $num by $( $ilane ),* wrapped_in $istate)>
            where
                T: Deref<Target = $reg>,
            {
                #[doc = "Disable interrupt " $num "."]
                #[doc = ""]
                #[doc = "For details, see Nordic's documentation on the [`INTENCLR`](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29_4_9#register.INTENCLR) register."]
                pub fn [< disable_interrupt_ $num >](self) -> Timer<T, W, C, S, enclose!(Disabled at $num by $( $ilane ),* wrapped_in $istate)> {
                    disable_interrupt!(self.timer, $num);
                    timer!(self.timer)
                }
            }

            impl<T, S, W, $( $ilane ),* , C> Timer<T, W, C, S, enclose!(Disabled at $num by $( $ilane ),* wrapped_in $istate)>
            where
                T: Deref<Target = $reg>,
            {
                #[doc = "Enable interrupt " $num "."]
                #[doc = ""]
                #[doc = "For details, see Nordic's documentation on the [`INTENSET`](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29_4_8#register.INTENSET) register."]
                pub fn [< enable_interrupt_ $num >](self) -> Timer<T, W, C, S, enclose!(Enabled at $num by $( $ilane ),* wrapped_in $istate)> {
                    enable_interrupt!(self.timer, $num);
                    timer!(self.timer)
                }
            }

            // Note that from the generics point of view, it would be sufficient
            // to simply introduce the impl block via
            // impl<T, S, W, I, C> Timer<T, S, W, I, C>
            // i.e. do not specify the interrupt state type.
            //
            // But because we have basic and extended timers, with the above
            // there is no way for the compiler to tell them apart (in the end,
            // there could be a T implementing both
            // T: Deref<Target = BasicRegBlock> AND
            // T: Deref<Target = ExtendedRegBlock>)
            // This makes the compiler not accept the code due to a duplicate
            // definition of the methods unpend_interrupt_0 through ..._3.
            //
            // Specifying the interrupt state type "kind of verbosely" gives the
            // compiler the opportunity to distinguish the unpend_interrupt_...
            // methods for basic and extended timers.
            impl<T, S, W, $( $ilane ),* , $ilane_prime, C> Timer<T, W, C, S, $istate<$( $ilane ),* , $ilane_prime>>
            where
                T: Deref<Target = $reg>,
            {
                #[doc = "Unpend interrupt " $num "."]
                #[doc = ""]
                #[doc = "For details, see Nordic's documentation on the [`EVENTS_COMPARE`](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29_4_6#register.EVENTS_COMPARE-0-5) register."]
                pub fn [< unpend_interrupt_ $num >](&mut self) {
                    unpend_interrupt!(self.timer, $num);
                }

                #[doc = "Set compare value " $num "."]
                #[doc = ""]
                #[doc = "For details, see Nordic's documentation on the [`CC`](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29_4_13#register.CC-0-5) register."]
                #[doc = ""]
                #[doc = "A note on safety: it is safe to set any `u32` compare value."]
                #[doc = "Depending on the timer's set bit width, not all bits will be used for comparison by the peripheral, though."]
                pub fn [< compare_against_ $num >](&mut self, val: u32) {
                    write_compare_value!(self.timer, $num, val);
                }

                #[doc = "Capture current timer value to CC register " $num "."]
                #[doc = ""]
                #[doc = "For details, see Nordic's documentation on the [`TASKS_CAPTURE`](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29_4_5#register.TASKS_CAPTURE-0-5) register."]
                pub fn [< capture_ $num >](&mut self) {
                    capture_value!(self.timer, $num);
                }
            }
        }
    }
}

macro_rules! define_basic_cc {
    ( $num:literal ) => {
        define_cc!(BasicRegBlock, BasicIRQState, $num, IA, IB, IC | ID);
    };
}

macro_rules! define_extended_cc {
    ( $num:literal ) => {
        define_cc!(
            ExtendedRegBlock,
            ExtendedIRQState,
            $num,
            IA,
            IB,
            IC,
            ID,
            IE | IF
        );
    };
}

define_basic_cc!(0);
define_basic_cc!(1);
define_basic_cc!(2);
define_basic_cc!(3);

define_extended_cc!(0);
define_extended_cc!(1);
define_extended_cc!(2);
define_extended_cc!(3);
define_extended_cc!(4);
define_extended_cc!(5);

impl<T, S, W, I, C> Timer<T, W, C, S, I>
where
    T: Instance,
{
    /// Clear/Reset the timer.
    ///
    /// This works both in `Started` as well as in `Stopped` state.
    /// See [Nordic's documentation on `TASKS_CLEAR`](https://infocenter.nordicsemi.com/topic/ps_nrf52840/timer.html?cp=5_0_0_5_29_4_3#register.TASKS_CLEAR) for details.
    pub fn reset(&mut self) {
        self.timer
            .as_timer0()
            .tasks_clear
            .write(|w| w.tasks_clear().set_bit());
    }
}
