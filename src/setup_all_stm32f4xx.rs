pub use stm32f4xx_hal::{
      pac::{CorePeripherals, Peripherals},
};

use stm32f4xx_hal::{
      pac::{I2C1, I2C2, TIM5 },
      timer::{Delay as halDelay, TimerExt},
      rcc::{Clocks, RccExt},
      i2c::I2c,   //this is a type
      gpio::{Output, PushPull, GpioExt,},
      gpio::{gpioc::{PC13 as LEDPIN}},
      prelude::*,
};

pub use embedded_hal::delay::DelayNs;

type Delay = halDelay<TIM5, 1000000_u32>;

pub type I2c1Type = I2c<I2C1>;
pub type I2c2Type = I2c<I2C2>;

pub use crate::led::LED;  // defines trait and default methods
pub type LedType = LEDPIN<Output<PushPull>>;
impl LED for LedType {}    

//   //////////////////////////////////////////////////////////////////////


pub fn setup_from_dp(dp: Peripherals) -> (I2c1Type, I2c2Type, LedType, Delay, Clocks) {

   let rcc = dp.RCC.constrain();
   let clocks = rcc.cfgr.freeze();
  
   //let gpioa = dp.GPIOA.split();
   let gpiob = dp.GPIOB.split();
   let gpioc   = dp.GPIOC.split();

   let scl = gpiob.pb8.into_alternate_open_drain(); 
   let sda = gpiob.pb9.into_alternate_open_drain(); 
   let i2c1 = I2c::new(dp.I2C1, (scl, sda), 400.kHz(), &clocks);

   let scl = gpiob.pb10.into_alternate_open_drain();
   let sda = gpiob.pb3.into_alternate_open_drain();
   let i2c2 = I2c::new(dp.I2C2, (scl, sda), 400.kHz(), &clocks);

   let mut led = gpioc.pc13.into_push_pull_output();
   led.off();

   let delay = dp.TIM5.delay(&clocks);

   (i2c1, i2c2, led, delay, clocks)
}


