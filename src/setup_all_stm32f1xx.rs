
pub use stm32f1xx_hal::{
      pac::{CorePeripherals, Peripherals},
};

use stm32f1xx_hal::{
      pac::{I2C1, I2C2, TIM3},
      rcc::{Clocks, RccExt},
      timer::{TimerExt, Delay},
      i2c::{DutyCycle, Mode as i2cMode, BlockingI2c, }, 
      gpio::{Output, PushPull, },
      gpio::{gpioc::{PC13 as LEDPIN}},
      prelude::*,
};


pub use embedded_hal::delay::DelayNs;

pub type I2c1Type = BlockingI2c<I2C1>;
pub type I2c2Type = BlockingI2c<I2C2>;

pub use crate::led::LED;  // defines trait and default methods
pub type LedType = LEDPIN<Output<PushPull>>;
impl LED for LedType {}    

//   //////////////////////////////////////////////////////////////////////

pub fn setup_from_dp(dp: Peripherals) -> (I2c1Type, I2c2Type, LedType, Delay<TIM3, 1000000_u32>, Clocks) {
    
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    //let mut afio = dp.AFIO.constrain();
    let clocks = rcc
        .cfgr
        //.use_hse(8.mhz()) // high-speed external clock 8 MHz on bluepill
        //.sysclk(64.mhz()) // system clock 8 MHz default, max 72MHz
        //.pclk1(32.mhz())  // system clock 8 MHz default, max 36MHz ?
        .freeze(&mut flash.acr);

    //let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();
    let mut gpioc = dp.GPIOC.split();


    let mut afio = dp.AFIO.constrain();

    // still only on branch = "rmp-new"
    let i2c1 = BlockingI2c::<I2C1>::new(
                  dp.I2C1
                  .remap(&mut afio.mapr),  // add this for PB8, PB9
                  (
                   gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh),  // scl 
                   gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh)   // sda
                  ),
                  i2cMode::Fast {frequency: 400.kHz(), duty_cycle: DutyCycle::Ratio16to9,},
                  &clocks, 1000, 10, 1000, 1000,);
    // or
    //let i2c1 = dp
    //    .I2C1
    //    .remap(&mut afio.mapr) // add this for PB8, PB9
    //    .blocking_i2c(
    //      (
    //       gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh), 
    //       gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh)
    //      ),
    //      Mode::Fast {frequency: 400.kHz(), duty_cycle: DutyCycle::Ratio16to9, },
    //      &clocks, 1000, 10, 1000, 1000,
    //    );
 

    let i2c2 = BlockingI2c::<I2C2>::new(
                 dp.I2C2,
                 (
                  gpiob.pb10.into_alternate_open_drain(&mut gpiob.crh), // scl 
                  gpiob.pb11.into_alternate_open_drain(&mut gpiob.crh), // sda
                 ),
                 i2cMode::Fast {frequency: 400_000_u32.Hz(), duty_cycle: DutyCycle::Ratio2to1,},
                 &clocks, 1000, 10, 1000, 1000,);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    led.off();

    //let delay = DelayType{};
    let delay = dp.TIM3.delay::<1000000_u32>(&clocks);

   (i2c1, i2c2, led, delay, clocks)
}

