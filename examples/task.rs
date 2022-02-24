//! examples/task.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m_semihosting::dbg;
use cortex_m::asm;
use panic_semihosting as _;
use lpc55_hal as hal;

#[rtic::app(device = crate::hal::raw, peripherals = true)]
const APP: () = {
    #[init(spawn = [foo])]
    fn init(c: init::Context) {
        dbg!("init!!!##############");
        c.spawn.foo().unwrap();
    }

    #[task(spawn = [bar, baz])]
    fn foo(c: foo::Context) {
        dbg!("foo - start");

        // spawns `bar` onto the task scheduler
        // `foo` and `bar` have the same priority so `bar` will not run until
        // after `foo` terminates
        c.spawn.bar().unwrap();

        dbg!("foo - middle");

        // spawns `baz` onto the task scheduler
        // `baz` has higher priority than `foo` so it immediately preempts `foo`
        c.spawn.baz().unwrap();

        dbg!("foo - end");
    }

    #[task]
    fn bar(_: bar::Context) {
        dbg!("bar", );

        dbg!("Bar: entering the endless loop...");
        loop {
            asm::wfi();
        dbg!("bar");
        }
        // debug::exit(debug::EXIT_SUCCESS);

    }

    #[task(priority = 2)]
    fn baz(_: baz::Context) {
        dbg!("baz");
    }

    // RTIC requires that unused interrupts are declared in an extern block when
    // using software tasks; these free interrupts will be used to dispatch the
    // software tasks.
    extern "C" {
        fn USB0();
        fn USB1();
    }
};
