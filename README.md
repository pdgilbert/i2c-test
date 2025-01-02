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
so multiple sensors can be used.

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
in place then in one window start
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

All examples and device crates use `embedded-hal v1.0.0` and  `embedded-hal v1.0.0` versions of MCU hals. 
All examples compile with  `stm32f1xx-hal` (bluepill), `stm32f4xx-hal` (blackpill stm32f401 and stm32f411),
and `stm32g4xx-hal` (stm32g474xE). (See the github "actions" tab to confirm if that is still true.)

Compiling unfortunately does not mean everything works. Results from manually running 
examples on hardware are summarized below. HALs and device crates may require some level of optimization in order
to work on hardware. (See https://github.com/stm32-rs/stm32f4xx-hal/issues/828.) The results below
are for tests with `release` and `dev` profiles set in `Cargo.toml` as

```
[profile.dev] 
debug = true 
lto = true 
opt-level = 1

[profile.release] 
debug = true 
lto = true 
opt-level = "s" 
```

### Summary of hardware testing

(Examples `aht20-bl-semi`, `aht20-dr-semi`, `aht20-em-semi`, `sht30-em-semi`, `shtc3-semi`,
 `xca9548a-aht20-bl`, `xca9548a-aht20-dr`, `xca9548a-aht20-em`, `xca9548a-sht30` and `xca9548a-shtc3` )

As Dec 30-31, 2024

 * Most examples are working using `stm32f4xx-hal#585dd0f` on blackpill `stm32f401` and `stm32f411`.
 * Many examples are working using `stm32f1xx-hal#6c5dc881` on bluepill (requires `--release`).
 * Some examples are working using `stm32g4xx-hal` on stm32g474xE. 

More details are recorded at https://github.com/pdgilbert/i2c-test/issues/1


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
