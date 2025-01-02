//! Continuously read temperature from SHT30 and with semihosting hprintln. Using sensor sht30-D.
//!
//! Dec 21, 2024
//!Compiles and runs on blackpill stm32f411.
//!Compiles on stm32g4xx (stm32g474xE) but run panicked after sen.measure()
//!   
//!Compiles on stm32f1xx (bluepill) but run panicked after sen.measure()
//!
//!

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_semihosting::hprintln;
//use cortex_m::asm;

use embedded_sht3x::{Repeatability::High, Sht3x, DEFAULT_I2C_ADDRESS}; 

#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_halt as _;

use cortex_m_rt::entry;

/////////////////////   hals

#[cfg(feature = "stm32f1xx")]
use stm32f1xx_hal::{
    timer::SysTimerExt,
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
    hprintln!("sht30-em-semi example");

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
    let mut sen  = Sht3x::new(&mut i2c1, DEFAULT_I2C_ADDRESS, &mut delay); // does not return Result
    hprintln!("Sensor started.");   
    sen.repeatability = High;
    
    loop {
        hprintln!("sen.measure()");
        let th = sen.single_measurement();   // Read humidity and temperature.
        
        match th {
             Ok(m)      => {hprintln!("{:.2}C  {:.2}% RH", m.temperature, m.humidity); },
             Err(e)     => {hprintln!("read error {:?}", e); }
             };

        delay2.delay_ms(5000); 
    }
}
