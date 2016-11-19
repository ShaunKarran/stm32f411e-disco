#![no_main]
#![no_std]

extern crate board;
extern crate stm32f411xx_memory_map;

use stm32f411xx_memory_map as peripheral;

#[no_mangle]
pub fn main() -> ! {
    let rcc = unsafe { peripheral::rcc_mut() };
    // Set the IOPAEN bit of the AHBENR register, which is in the RCC register
    // block, to 1
    rcc.ahb1enr.modify(|_, w| w.gpioden(true));

    let gpiod = unsafe { peripheral::gpiod_mut() };

    gpiod.moder.modify(|_, w| {
        w.moder12(0b01)
            .moder13(0b01)
            .moder14(0b01)
            .moder15(0b01)
    });

    let bsrr = &peripheral::gpiod().bsrr;

    loop {
        bsrr.write(|w| w.bs13(true));
        for x in 0..30000 {}
        bsrr.write(|w| w.br13(true));
        for x in 0..30000 {}
    }
}
