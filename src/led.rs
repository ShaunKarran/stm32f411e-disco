//! Easy use of LEDs.

use peripheral;

/// All the LEDs.
pub static LEDS: [Led; 4] = [Led { i: 12 },
                             Led { i: 13 },
                             Led { i: 14 },
                             Led { i: 15 }];

/// A single LED.
pub struct Led {
    i: u8,
}

impl Led {
    /// Turns the LED on.
    pub fn on(&self) {
        let bsrr = &peripheral::gpiod().bsrr;
        match self.i {
            12 => bsrr.write(|w| w.bs12(true)),
            13 => bsrr.write(|w| w.bs13(true)),
            14 => bsrr.write(|w| w.bs14(true)),
            15 => bsrr.write(|w| w.bs15(true)),
            _ => {}
        }
    }

    /// Turns the LED off.
    pub fn off(&self) {
        let bsrr = &peripheral::gpiod().bsrr;
        match self.i {
            12 => bsrr.write(|w| w.br12(true)),
            13 => bsrr.write(|w| w.br13(true)),
            14 => bsrr.write(|w| w.br14(true)),
            15 => bsrr.write(|w| w.br15(true)),
            _ => {}
        }
    }
}

/// Initializes the necessary stuff to drive the LEDs on and off.
///
/// # Safety
///
/// - Must be called once
/// - Must be called in an interrupt-free environment
pub unsafe fn init() {
    let gpiod = peripheral::gpiod_mut();
    let rcc = peripheral::rcc_mut();

    // RCC: Enable GPIOD
    rcc.ahb1enr.modify(|_, w| w.gpioden(true));

    // GPIOD: Configure pins 12-15 as outputs
    gpiod.moder.modify(|_, w| {
        w.moder12(0b01)
            .moder13(0b01)
            .moder14(0b01)
            .moder15(0b01)
    });
}

/// An enum over the LEDs, each LED has associated to it a colour.
pub enum Colour {
    /// Green
    Green,
    /// Orange
    Orange,
    /// Red
    Red,
    /// Blue
    Blue,
}

impl Colour {
    /// Turns on this LED.
    pub fn on(&self) {
        match *self {
            Colour::Green => LEDS[0].on(),
            Colour::Orange => LEDS[1].on(),
            Colour::Red => LEDS[2].on(),
            Colour::Blue => LEDS[3].on(),
        }
    }

    /// Turns off this LED.
    pub fn off(&self) {
        match *self {
            Colour::Green => LEDS[0].off(),
            Colour::Orange => LEDS[1].off(),
            Colour::Red => LEDS[2].off(),
            Colour::Blue => LEDS[3].off(),
        }
    }
}
