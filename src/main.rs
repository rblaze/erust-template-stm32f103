#![no_std]
#![no_main]

use cortex_m_rt::entry;
use rtt_target::debug_rprintln;
use stm32f1xx_hal::pac as _;

#[cfg(debug_assertions)]
use rtt_target::rtt_init_print;

use panic_probe as _;
// use panic_halt as _;

#[entry]
fn main() -> ! {
    #[cfg(debug_assertions)]
    rtt_init_print!();

    debug_rprintln!("starting");

    loop {}
}
