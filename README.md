# Rust embedded-hal 1.0.0  i2c temperature/humidity sensor example testing 

## Summary

This `Rust` crate-like repository has some `embedded-hal 1.0.0` `i2c` sensor examples 
using `stm32f1xx_hal` `stm32f4xx_hal` and `stm32g4xx_hal`. 
Setup functions in `src/` do all board/MCU/hal initialization.
For example, the setup for blackpill boards is done with the file `src/setup_all_stm32f4xx.rs`.
The application part of each example is generic code that should work with all MCUs. 

The `*semi` examples use semihosting to display sensor readings. 
This focuses on the sensor crate and eliminates other crates that may cause complications.
The `xca*` examples include an `ssd1306` display and  a multiplexer
so muliple sensors can be used.

Results from compiling examples are reported in the repository 'Actions' tab
https://github.com/pdgilbert/rust-integration-testing/actions. (This is a small subset of
examples at https://github.com/pdgilbert/rust-integration-testing/actions.)
The Action jobs provide information from `cargo build`. 
Also, `cargo tree` is shown for debugging purposes.
Results from manually running examples on hardware are reported further below.


## Hardware Notes

The repository has files for  link/flash/run/debug
(e.g. `.cargo/config`, `build.rs` and `memory.x`). These are not used in the github workflows.
but are useful for linking and running as below.

## Building

In a cloned version of this repository the examples can be built manually by setting one of these lines:
```
               environment variables for cargo                       openocd         embed        test board and processor
  _____________________________________________________________     _____________  _____________   ___________________________
  export HAL=stm32f1xx MCU=stm32f103   TARGET=thumbv7m-none-eabi    PROC=stm32f1x  CHIP=STM32F103C8  # bluepill         Cortex-M3
  export HAL=stm32f4xx MCU=stm32f401   TARGET=thumbv7em-none-eabihf PROC=stm32f4x  CHIP=STM32F4x  # blackpill-stm32f401 Cortex-M4
  export HAL=stm32f4xx MCU=stm32f411   TARGET=thumbv7em-none-eabihf PROC=stm32f4x  CHIP=STM32F4x  # blackpill-stm32f411 Cortex-M4
  export HAL=stm32g4xx MCU=stm32g474xE TARGET=thumbv7em-none-eabihf PROC=stm32g4x  CHIP=STM32G4x  # weact-stm32g474CEU6 Cortex-M4
```
then to build
```
cargo build --no-default-features --target $TARGET --features $MCU,$HAL --example xxx
```
where `xxx` is replaced by one of the example names, such as
```
cargo build --no-default-features --target $TARGET --features $MCU,$HAL --example aht20-dr-semi

```

## Loading

For example, with `openocd`, `gdb`, `.cargo/config` with needed runners, and an appropriate probe  
in place then in one window run
```
openocd -f interface/stlink.cfg -f target/$PROC.cfg 
```

In another window do
```
cargo  run --target $TARGET --features $HAL,$MCU --example xxx  [ --release]
```
The `--release` will be needed if code is too big for memory.


## Sensor Crate Notes 

I have a project that needs several similar sensors on one i2c bus.  Since sensors often 
have a hardware fixed i2c address, this requires multiplexing with something like a tca9548a or pca9548a. 
Here is a wish list of senser crate properties, some necessary and some nice, 
for my project with multiplexed sensors :

 - Be no_std.
 - Use embedded-hal v1. (This seems necessary if the multiplex crate does.)
 - The binary should not be overly big, so other things can also be done on an MCU with limited flash.
 - Sensor intialization should return a Result so that it is possible to identify
     when some multiplex ports do not have sensors attached.
 - Allow for software reset (if the sensor does) so individual sensors can be reset without a power cycle.
 - A syncronous (blocking) version is necessary (at the moment because of my rtic difficulties).
     The feature of getting both sync and async from the same crate is nice for future considerations.
 - Preferably the sensor driver should borrow rather than consume `delay`. If `delay` is
   consumed then a large number of delays are used with multiplexing . This can
   be done with cortex_m::asm::delay but that adds unnecessary complication (size?).
 - A release version of the device crate is preferred, rather than just a repository version.



## Run Testing

Compiling unfortunately does not mean everything works. Results from manually running 
examples on hardware are as follows:


### Summary of issues as of Dec. 2024:

(Using `stm32f1xx-hal v0.10.0  and also https://github.com/stm32-rs/stm32f1xx-hal#5fc8b999,
`stm32f4xx-hal v0.22.1` and also https://github.com/stm32-rs/stm32f4xx-hal#ed88ea13, 
`stm32g4xx-hal` from https://github.com/techmccat/stm32g4xx-hal#9462f4cd branch `hal-1` and also
 https://github.com/pdgilbert/stm32g4xx-hal branch `hal-1`. )

All examples use `embedded-hal v1.0.0 and  `embedded-hal v1.0.0` versions of MCU hals. 
All examples compile with  stm32f1xx (bluepill), stm32f4xx (blackpill stm32f401 and stm32f411) and stm32g4xx (stm32g474xE).

The `xca9548a*` examples compile but all fail to run, even when the sensor `*semi` crate works.
Running on `bluepill` requires `--release` so it is hard to debug. I have had no success running
with `bluepill` so far.  Blackpill stm32f401 and blackpill stm32f411 produce similar results and
are referred to simply as blackpill.


#### Example `aht20-dr-semi` 

(using crate `aht20-driver v2.0.0` and also git commit `#fd81e034` https://github.com/anglerud/aht20-driver)
Runs with `stm32g4xx` on `stm32g474xE`. 
[NB  *** if scl/sda wire are reversed gives: panicked at examples/aht20-dr-semi.rs:84:49:
called `Result::unwrap()` on an `Err` value: I2c(Nack) ]

With `stm32f4xx` (blackpill) it does not return from the first call to `init the sensor...`. 

Remote target In: aht20_dr_semi::__cortex_m_rt_m* L84   PC: 0x8002802
84      let mut aht = aht20_uninit.init(&mut delay).unwrap();  

Remote target In: aht20_driver::AHT20<stm32f4xx_* L361  PC: 0x80022be
361          while !self.check_status()?.is_calibrated() {

Remote target In: aht20_driver::AHT20<stm32f4xx_* L389  PC: 0x8002192
389          self.i2c    

Remote target In: stm32f4xx_hal::i2c::hal_1::blo* L11   PC: 0x8001872
11              self.read(addr, buffer)   

Remote target In: stm32f4xx_hal::i2c::I2c<stm32f* L439  PC: 0x800116a 
439          self.read_inner(addr.into(), buffer, true) 

Remote target In: stm32f4xx_hal::i2c::I2c<stm32f* L476  PC: 0x8000a68 
476              while self.i2c.cr1().read().stop().bit_is_set() {}    

#### Example aht20-bl-semi

(using crate `aht20` https://github.com/blueluna/aht20#07a72ca9e725bdc9d0ae6ec4ec265ed53a07d69f")
Runs with stm32g4xx on stm32g474xE. 
With stm32f4xx (blackpill) it does not return from the first call to `aht.read()`.  ..  an initialization call to the hal.


#### Example aht20-em-semi 

(using crate `embedded-aht20 v0.1.3` https://github.com/ghismary/embedded-aht20)
Runs with stm32g4xx on stm32g474xE. 
With stm32f4xx (blackpill) it does not return from Start the sensor  ... an initialization call to the hal.



#### Example shtc3-semi

(using crate `shtcx-rs` https://github.com/dbrgn/shtcx-rs)
Runs with stm32f4xx (blackpill). 
With stm32g4xx on stm32g474xE  it does not return from the first call to sen.measure()

85  sen.measure()  Normal mode 
Info : halted: PC: 0x080068f4
panicked at examples/shtc3-semi.rs:85:65:
Normal mode measurement failed: I2c(Nack)
Sometimes works stepping in gdb. Possibly delay needs to be longer?


#### Example sht30-em-semi

(using crate `embedded-sht3x` https://gitlab.com/ghislainmary/embedded-sht3x/)
Runs with stm32f4xx (blackpill). 
With stm32g4xx on stm32g474xE it does not return from the first call to sen.single_measurement().unwrap().
81          let th = sen.single_measurement().unwrap(); 
223          self.i2c.transaction(self.address, &mut operations)?; 
embedded_hal::i2c::{impl#7}::t* L430 
   freezes ... halted: PC: 0x080007a8



## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
