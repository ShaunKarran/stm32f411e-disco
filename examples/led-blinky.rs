#![no_main]
#![no_std]

extern crate stm32f411e_disco;

use stm32f411e_disco::led;

#[no_mangle]
pub fn main() -> ! {
    unsafe { led::init(); }

    let red_led = led::Colour::Red;
    let blue_led = led::Colour::Blue;
    let green_led = led::Colour::Green;
    let orange_led = led::Colour::Orange;

    loop {
        red_led.on();
        for _ in 0..10000 {}
        red_led.off();
        blue_led.on();
        for _ in 0..10000 {}
        blue_led.off();
        green_led.on();
        for _ in 0..10000 {}
        green_led.off();
        orange_led.on();
        for _ in 0..10000 {}
        orange_led.off();
    }
}
