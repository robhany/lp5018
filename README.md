![build_workflow](https://github.com/robhany/lp5018/actions/workflows/rust.yml/badge.svg)

# lp5018

This crate is a no_std driver for the lp5018 i2c LED driver.

## Datasheet

https://www.ti.com/lit/gpn/lp5018

## About this driver
This driver does not support all features of the LED driver such as power saving mode, programmable
bank support or auto-increment for writing or reading with one transition.
It supports setting the brightness of each output.

## Usage
Add this to your Cargo.toml:

```toml
[dependencies]
lp5018 = "0.1.2"
```

And this to your main.rs

```rust
    let mut led_controller_enable_pin = some_pin;
    let mut led_controller = lp5018::LedDriver::new();
    led_controller.set_address(true, true);
    led_controller_enable_pin.set_high().unwrap();
    timer.delay_ms(500_u32);
    led_controller.reset(&mut i2c).unwrap();
    led_controller.init_device(&mut i2c).unwrap();

    loop {
        //turn on LED
        led_controller
            .change_intensity_for_output(&mut i2c, Out00, 0xff)
            .unwrap();
        timer.delay_ms(500_u32);
        //turn off LED
        led_controller
            .change_intensity_for_output(&mut i2c, Out00, 0x00)
            .unwrap();
        timer.delay_ms(500_u32);
    }
```
