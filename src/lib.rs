//! A crate to hack the STM32F411E-DISCO board!

#![feature(asm)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(naked_functions)]
#![no_std]

extern crate compiler_builtins_snapshot;
#[macro_reexport(bkpt)]
#[macro_use]
extern crate cortex_m;
extern crate r0;
extern crate volatile_register;

pub extern crate stm32f411xx_memory_map as peripheral;

mod lang_items;

pub mod exception;
pub mod interrupt;
pub mod led;

// "Pre `main`" initialization routine
fn init() {}
