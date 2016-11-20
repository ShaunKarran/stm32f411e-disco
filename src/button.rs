//! Easy use of buttons.

use peripheral;

/// The user button.
pub static BUTTONS: [Button; 1] = [Button { i: 0 }];

/// A single button.
pub struct Button {
    i: u8,
}

impl Button {
    /// Read the state of the button.
    pub fn pressed(&self) -> bool {
        let idr = &peripheral::gpioa().idr;

        match self.i {
            0 => idr.read().idr0(),
            _ => false
        }
    }
}

/// Initializes the necessary stuff to enable the button.
///
/// # Safety
///
/// - Must be called once
/// - Must be called in an interrupt-free environment
pub unsafe fn init() {
    let rcc = peripheral::rcc_mut();

    // RCC: Enable GPIOA
    rcc.ahb1enr.modify(|_, w| w.gpioaen(true));
}

/// An enum over the Buttons, each Button is associated but its name.
pub enum Buttons {
    /// User
    User,
}

impl Buttons {
    /// Read the state of this button.
    pub fn pressed(&self) -> bool {
        match *self {
            Buttons::User => BUTTONS[0].pressed()
        }
    }
}
