# lp5018

This crate is a no_std driver for the lp5018 i2c LED driver.

## Datasheet

https://www.ti.com/lit/gpn/lp5018

## About this driver
This driver does not support all features of the LED driver such as power saving mode, programmable
bank support or auto-increment for writing or reading with one transition.
It supports setting the brightness of each output.
