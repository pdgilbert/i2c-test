//! Continuously read temperature from SHTC3 and with semihosting hprintln. SHTC3 sensor.
//!
//! Dec 21, 2024
//!Compiles and runs on blackpill stm32f411.
//!Compiles on stm32g4xx (stm32g474xE) but run panicked at shtcx-semi.rs:86:65:
//!    Normal mode measurement failed: I2c(Nack)
//!Compiles on stm32f1xx (bluepill) but run panicked at shtcx-semi.rs after Sensor started.

//! The "semi" examples simplify testing of the sensor crate alone, without display complications.

//! Using crate embedded-aht20

#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[cfg(debug_assertions)]
use panic_semihosting as _;

#[cfg(not(debug_assertions))]
use panic_halt as _;

use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;
//use cortex_m::asm;

use shtcx::{LowPower, PowerMode};


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
use i2c_test::setup::{CorePeripherals, Peripherals, DelayNs, LED};

#[entry]
fn main() -> ! {
    hprintln!("shtcx-semi example");

    let dp = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();

    let (mut i2c1, _i2c2, mut led, mut delay, clocks) = setup::setup_from_dp(dp);

    let mut delay2 = cp.SYST.delay(&clocks); 

    hprintln!("delay.delay_ms(2000)");
    delay2.delay_ms(2000);    

    led.on();
    hprintln!("delay.delay_ms(10000)");
    delay.delay_ms(2000);    
    led.off();

    hprintln!("Start the sensor...");
    // Start the sensor.   address 0x38 cannot be changed

    // asm::bkpt();  
    let mut sen  = shtcx::shtc3(&mut i2c1);
    hprintln!("Sensor started.");    // does not return Result
    sen.wakeup(&mut delay).expect("Wakeup failed");  // handle Result
    hprintln!("Sensor awake.");    

    loop {
        hprintln!("loop i");      
        
        hprintln!("sen.measure()  Normal mode ");
        let th = sen.measure(PowerMode::NormalMode, &mut delay).expect("Normal mode measurement failed");

        // try this
        //hprintln!("sen.measure() PowerMode  mode ");
        //sen.sleep().expect("Sleep command failed");
        //let th = sen.measure(PowerMode::NormalMode, &mut delay).expect("PowerMode mode measurement failed");
        
        hprintln!("{:.2}C  {:.2}% RH", th.temperature.as_degrees_celsius(), th.humidity.as_percent());

        delay2.delay_ms(5000); 
    }
}
