#![no_main]
#![no_std]

extern crate stm32f411e_disco;

use stm32f411e_disco::led;

#[no_mangle]
pub fn main() -> ! {
    unsafe { led::init(); }

    let red_led = led::Colour::Red;

    loop {
        red_led.on();
        for _ in 0..30000 {}
        red_led.off();
        for _ in 0..30000 {}
    }
}
