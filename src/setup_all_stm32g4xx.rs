pub use stm32g4xx_hal::{
      pac::{CorePeripherals, Peripherals},
};

use stm32g4xx_hal::{
      pac::{ I2C1, I2C2, TIM3},
      rcc::{RccExt, Clocks},
      i2c::I2c,   //this is a type
      gpio::{Output, PushPull, GpioExt},
      prelude::*,
      i2c::{Config},
      time::{ExtU32, RateExtU32},
      timer::Timer,
      timer::CountDownTimer,
      delay::DelayFromCountDownTimer,
      gpio::{AlternateOD, 
             gpioa::{ PA8,},
             gpiob::{ PB8, PB9},
             gpioc::{PC4, PC6 as LEDPIN}},  // weact-stm32g474CEU6 has onboard led on PC6
};


pub use embedded_hal::delay::DelayNs;

pub type I2c1Type = I2c<I2C1, PB9<AlternateOD<4_u8>>, PB8<AlternateOD<4_u8>>>;
pub type I2c2Type = I2c<I2C2, PA8<AlternateOD<4_u8>>, PC4<AlternateOD<4_u8>>>;

pub use crate::led::LED;  // defines trait and default methods
pub type LedType = LEDPIN<Output<PushPull>>;
impl LED for LedType {  // not default
        fn on(&mut self) -> () {
            self.set_high().unwrap()
        }
        fn off(&mut self) -> () {
            self.set_low().unwrap()
        }
    }


//   //////////////////////////////////////////////////////////////////////


pub fn setup_from_dp(dp: Peripherals) -> (I2c1Type, I2c2Type, LedType, DelayFromCountDownTimer<CountDownTimer<TIM3>>, Clocks) {
   
   let mut rcc = dp.RCC.constrain();
   let clocks = rcc.clocks;  // not sure if this is right

   let gpioa = dp.GPIOA.split(&mut rcc);
   let gpiob = dp.GPIOB.split(&mut rcc);
   let gpioc = dp.GPIOC.split(&mut rcc);

   let sda = gpiob.pb9.into_alternate_open_drain(); 
   let scl = gpiob.pb8.into_alternate_open_drain(); 
   let i2c1 = dp.I2C1.i2c(sda, scl, Config::new(400.kHz()), &mut rcc); // NOTE ORDER OF SDA,SCL REVERSED FROM stm32f4xx

   let sda = gpioa.pa8.into_alternate_open_drain(); 
   let scl = gpioc.pc4.into_alternate_open_drain();
   let i2c2 = dp.I2C2.i2c(sda, scl, Config::new(400.kHz()), &mut rcc); // NOTE ORDER OF SDA,SCL REVERSED FROM stm32f4xx

   let mut led = gpioc.pc6.into_push_pull_output();
   led.off();

   let timerx = Timer::new(dp.TIM3, &clocks);
   let  delay = DelayFromCountDownTimer::new(timerx.start_count_down(100.millis()));

   (i2c1, i2c2, led, delay, clocks)
}

