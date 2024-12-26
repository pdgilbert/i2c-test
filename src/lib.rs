// The library code is only setup functions:

#![no_std]

#![feature(type_alias_impl_trait)]   //  for impl Trait` in type aliases is unstable

#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_halt as _;


pub mod alt_delay;

pub mod led;   // has trait and default impl

pub mod  setup;


#[cfg(feature = "stm32f1xx")]
pub mod  setup_all_stm32f1xx;

#[cfg(feature = "stm32f4xx")]
pub mod  setup_all_stm32f4xx;

#[cfg(feature = "stm32g4xx")]
pub mod  setup_all_stm32g4xx;

