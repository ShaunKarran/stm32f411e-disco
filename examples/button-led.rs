#![no_main]
#![no_std]

extern crate stm32f411e_disco;

use stm32f411e_disco::button;
use stm32f411e_disco::led;

#[no_mangle]
pub fn main() -> ! {
    unsafe { button::init(); }
    unsafe { led::init(); }

    let green_led = led::Colour::Green;
    let user_button = button::Buttons::User;

    loop {
        if user_button.pressed() {
            green_led.on();
        } else {
            green_led.off();
        }
    }
}
