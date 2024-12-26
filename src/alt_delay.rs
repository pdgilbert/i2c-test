#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_halt as _;

use embedded_hal::delay::DelayNs;

// A delay is consumed by some sensors on initializations. 
// asm::delay is not an accurate timer but gives a delay at least number of indicated clock cycles.

use cortex_m::asm::delay; // argment in clock cycles so (5 * CLOCK) cycles gives aprox 5 second delay

//should be set for board not for HAL

#[cfg(feature = "stm32f1xx")]
pub const ALTCLOCK: u32 = 8_000_000;   // really 8_000_000;  but not providing enough delay for DHT-11

#[cfg(feature = "stm32f4xx")]
pub const  ALTCLOCK: u32 = 16_000_000;

#[cfg(feature = "stm32g4xx")]
pub const  ALTCLOCK: u32 = 16_000_000;


pub struct AltDelay {}

impl DelayNs for AltDelay {
    fn delay_ns(&mut self, t:u32) {
        delay((t as u32) * (ALTCLOCK / 1_000_000_000)); 
    }
    fn delay_us(&mut self, t:u32) {
        delay((t as u32) * (ALTCLOCK / 1_000_000)); 
    }
    fn delay_ms(&mut self, ms: u32) {
        delay((ms as u32) * (ALTCLOCK / 1000)); 
    }
}
