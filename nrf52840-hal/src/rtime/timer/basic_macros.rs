macro_rules! timer {
    ( $timer:expr ) => {
        Timer {
            timer: $timer,
            w: PhantomData,
            c: PhantomData,
            s: PhantomData,
            i: PhantomData,
        }
    };
}

macro_rules! stop_timer {
    ( $timer:expr ) => {
        $timer.tasks_stop.write(|w| w.tasks_stop().set_bit());
    };
}

macro_rules! set_width {
    ( $timer:expr, $width:ty ) => {
        $timer.as_timer0().bitmode.write(|w| <$width>::set(w));
    };
}

macro_rules! ensure_width_32 {
    ( $timer:expr ) => {
        set_width!($timer, W32);
    };
}

macro_rules! set_mode {
    ( $timer:expr, $mode:ident ) => {
        paste::paste! {
            $timer.mode.write(|w| w.mode(). [< $mode >]());
        }
    };
}

macro_rules! ensure_prescale_0 {
    ( $timer:expr ) => {
        $timer.prescaler.write(|w| unsafe { w.bits(P0::VAL) });
    };
}

macro_rules! disable_interrupt {
    ( $timer:expr, $num:literal ) => {
        paste::paste! {
            $timer.intenclr.write(|w| w.[< compare $num >]().set_bit());
        }
    };
}

// Note that we can chain up multiple disables into a single call to write,
// which is why in below definition we do *not* call `disable_interrupt!`.
macro_rules! disable_interrupts {
    ( $timer:expr, $( $i:literal ),+ ) => {
        paste::paste! {
            $timer.intenclr.write(|w| {
                w
                $(
                    .[< compare $i >]()
                    .set_bit()
                )+
            });
        }
    }
}

macro_rules! enable_interrupt {
    ( $timer:expr, $num:literal ) => {
        paste::paste! {
            $timer.intenset.write(|w| w.[< compare $num >]().set_bit());
        }
    };
}

macro_rules! unpend_interrupt {
    ( $timer:expr, $num:literal ) => {
        $timer.events_compare[$num].write(|w| w);
    };
}

macro_rules! unpend_interrupts {
    ( $timer:expr, $( $i:literal ),+ ) => {
        $(
            unpend_interrupt!( $timer, $i );
        )+
    };
}

macro_rules! write_compare_value {
    ( $timer:expr, $num:literal, $val:ident) => {
        $timer.cc[$num].write(|w| unsafe { w.cc().bits($val) });
    };
}

macro_rules! capture_value {
    ( $timer:expr, $num:literal ) => {
        $timer.tasks_capture[$num].write(|w| w.tasks_capture().set_bit());
    };
}
