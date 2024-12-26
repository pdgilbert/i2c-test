//! Continuously read temperature from AHT20 and with semihosting hprintln.
//!
//! The "semi" examples simplify testing of the sensor crate alone, without display complications.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_semihosting::hprintln;
//use cortex_m::asm;

use aht20_bl::{Aht20};

#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_halt as _;

use cortex_m_rt::entry;

/////////////////////   hals

#[cfg(feature = "stm32f1xx")]
use stm32f1xx_hal::{
    timer::{SysTimerExt, },
};

#[cfg(feature = "stm32f4xx")]
use stm32f4xx_hal::{
    timer::SysTimerExt,
};

#[cfg(feature = "stm32g4xx")]
use stm32g4xx_hal::{
    delay::SYSTDelayExt,
};


///////////////////// 

use i2c_test::setup;
use i2c_test::setup::{Peripherals, DelayNs,};
use i2c_test::setup::{CorePeripherals};

#[entry]
fn main() -> ! {
//    hprintln!("AHT20-em example");

    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let (mut i2c1, _i2c2, _led, mut delay, clocks) = setup::setup_from_dp(dp);

    let mut delay2 = cp.SYST.delay(&clocks); 

    hprintln!("delay.delay_ms(2000)");
    delay.delay_ms(2000);    

    hprintln!("delay2.delay_ms(2000)");
    delay2.delay_ms(2000);    

    hprintln!("Start the sensor...");
    // Start the sensor.   address 0x38 cannot be changed

//  asm::bkpt(); 
    let mut aht = Aht20::new(&mut i2c1, &mut delay);
    hprintln!("Sensor started.");

    loop {
        hprintln!("aht.read()"); 
        let (h, t) = aht.read().unwrap();
        hprintln!("{:.3}C  {}% RH", t.celsius(), h.rh());

        delay2.delay_ms(5000); 
    }
}
