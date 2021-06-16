#![no_std]
extern crate embedded_hal as hal;

use crate::Output::*;
use hal::blocking::i2c::{Write, WriteRead};

//settings in binary format, so they are comparable with the values from the datasheet
const BROADCAST_ADDRESS: u8 = 0b11_1100;
const BASE_ADDRESS: u8 = 0b10_1000;
const ADDR0_PIN_HIGH: u8 = 0b0001;
const ADDR1_PIN_HIGH: u8 = 0b0010;
const CONFIG_0_CHIP_ENABLE: u8 = 0b0100_0000;
// const CONFIG_1_LED_GLOBAL_OFF: u8 = 0b0000_0001;
// const CONFIG_1_INCREASE_MAX_CURRENT_TO_35MA: u8 = 0b0000_0010;
const CONFIG_1_PWM_DITHERING_ENABLED: u8 = 0b0000_0100;
// const CONFIG_1_AUTO_INCREMENT_ENABLED: u8 = 0b0000_1000;
// const CONFIG_1_POWER_SAVE_ENABLED: u8 = 0b0001_0000;
const CONFIG_1_LOG_SCALE_ENABLED: u8 = 0b0010_0000;

#[repr(u8)]
#[derive(Clone)]
enum ConfigRegisters {
    DeviceConfig0 = 0x00,
    DeviceConfig1,
    Reset = 0x27,
}

impl ConfigRegisters {
    fn as_u8_value(&self) -> u8 {
        self.clone() as u8
    }
}

#[repr(u8)]
#[derive(Clone)]
pub enum Output {
    Out00 = 0x0F,
    Out01,
    Out02,
    Out03,
    Out04,
    Out05,
    Out06,
    Out07,
    Out08,
    Out09,
    Out10,
    Out11,
    Out12,
    Out13,
    Out14,
    Out15,
    Out16,
    Out17,
}

impl Output {
    fn as_u8_value(&self) -> u8 {
        self.clone() as u8
    }
}

pub struct LedDriver {
    address: u8,
}

impl Default for LedDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl LedDriver {
    pub fn new() -> Self {
        LedDriver {
            address: BROADCAST_ADDRESS,
        }
    }

    pub fn set_address(&mut self, addr0_is_high: bool, addr1_is_high: bool) {
        self.address = BASE_ADDRESS;
        if addr0_is_high {
            self.address |= ADDR0_PIN_HIGH;
        }
        if addr1_is_high {
            self.address |= ADDR1_PIN_HIGH;
        }
    }

    pub fn is_turned_on<I2C, E>(&self, i2c: &mut I2C) -> Result<bool, E>
    where
        I2C: Write<Error = E> + WriteRead<Error = E>,
    {
        let mut buf = [0_u8];
        i2c.write_read(
            self.address,
            &[ConfigRegisters::DeviceConfig0.as_u8_value()],
            &mut buf,
        )?;

        Ok(buf.first().unwrap() & CONFIG_0_CHIP_ENABLE == CONFIG_0_CHIP_ENABLE)
    }

    pub fn init_device<I2C, E>(&self, i2c: &mut I2C) -> Result<(), E>
    where
        I2C: Write<Error = E> + WriteRead<Error = E>,
    {
        // reset all values
        self.reset(i2c)?;
        // enable controller
        self.write(
            i2c,
            &[
                ConfigRegisters::DeviceConfig0.as_u8_value(),
                CONFIG_0_CHIP_ENABLE,
            ],
        )?;
        // enable dithering for LEDs and enable use of logarithmic scale when setting brightness of LED
        self.write(
            i2c,
            &[
                ConfigRegisters::DeviceConfig1.as_u8_value(),
                CONFIG_1_PWM_DITHERING_ENABLED | CONFIG_1_LOG_SCALE_ENABLED,
            ],
        )
    }

    pub fn set_all<I2C, E>(&self, i2c: &mut I2C, intensity: u8) -> Result<(), E>
    where
        I2C: Write<Error = E> + WriteRead<Error = E>,
    {
        self.change_intensity_for_output(i2c, Out00, intensity)?;
        self.change_intensity_for_output(i2c, Out01, intensity)?;
        self.change_intensity_for_output(i2c, Out02, intensity)?;
        self.change_intensity_for_output(i2c, Out03, intensity)?;
        self.change_intensity_for_output(i2c, Out04, intensity)?;
        self.change_intensity_for_output(i2c, Out05, intensity)?;
        self.change_intensity_for_output(i2c, Out06, intensity)?;
        self.change_intensity_for_output(i2c, Out07, intensity)?;
        self.change_intensity_for_output(i2c, Out08, intensity)?;
        self.change_intensity_for_output(i2c, Out09, intensity)?;
        self.change_intensity_for_output(i2c, Out10, intensity)?;
        self.change_intensity_for_output(i2c, Out11, intensity)?;
        self.change_intensity_for_output(i2c, Out12, intensity)?;
        self.change_intensity_for_output(i2c, Out13, intensity)?;
        self.change_intensity_for_output(i2c, Out14, intensity)?;
        self.change_intensity_for_output(i2c, Out15, intensity)?;
        self.change_intensity_for_output(i2c, Out16, intensity)?;
        self.change_intensity_for_output(i2c, Out17, intensity)
    }

    pub fn change_intensity_for_output<I2C, E>(
        &self,
        i2c: &mut I2C,
        output_pin: Output,
        intensity: u8,
    ) -> Result<(), E>
    where
        I2C: Write<Error = E> + WriteRead<Error = E>,
    {
        self.write(i2c, &[output_pin.as_u8_value(), intensity])
    }

    pub fn reset<I2C, E>(&self, i2c: &mut I2C) -> Result<(), E>
    where
        I2C: Write<Error = E> + WriteRead<Error = E>,
    {
        self.write(i2c, &[ConfigRegisters::Reset.as_u8_value(), 0xFF])
    }

    fn write<I2C, E>(&self, i2c: &mut I2C, bytes: &[u8]) -> Result<(), E>
    where
        I2C: Write<Error = E> + WriteRead<Error = E>,
    {
        i2c.write(self.address, bytes)
    }
}
