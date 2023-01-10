#![no_std]
#![no_main]
#![deny(unsafe_code)]

use panic_rtt_target as _;


#[rtic::app(device = microbit::pac, peripherals = true, dispatchers = [SWI0])]
mod app {
    use rtt_target::{rprintln, rtt_init_print};
    use microbit::{hal::timer::Timer, Board, pac, hal};
    use microbit::pac::Interrupt;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        (
            Shared{},
            Local {},
            init::Monotonics(),
        )
    }

    #[task()]
    fn foo(mut cx: foo::Context) {
    }

    #[idle()]
    fn idle(cx: idle::Context) -> ! {
        loop {
            cortex_m::asm::nop();
        }
    }
}
