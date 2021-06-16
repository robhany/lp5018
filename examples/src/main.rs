#![no_main]
#![no_std]
extern crate cortex_m;
#[macro_use(exception)]
extern crate cortex_m_rt as rt;
extern crate embedded_hal as ehal;
extern crate lp5018;
extern crate panic_semihosting;
extern crate stm32l4xx_hal as hal;

use hal::{delay::Delay, i2c::I2c, prelude::*};
use lp5018::Output::Out00;
use rt::{entry, ExceptionFrame};

#[entry]
fn main() -> ! {
    let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
    let hal_peripherals = hal::stm32::Peripherals::take().expect("failed to get stm32 peripherals");
    let mut flash = hal_peripherals.FLASH.constrain();
    let mut rcc = hal_peripherals.RCC.constrain();
    let mut pwr = hal_peripherals.PWR.constrain(&mut rcc.apb1r1);
    let clocks = rcc
        .cfgr
        .sysclk(80.mhz())
        .pclk1(80.mhz())
        .pclk2(80.mhz())
        .freeze(&mut flash.acr, &mut pwr);
    let mut gpioa = hal_peripherals.GPIOA.split(&mut rcc.ahb2);
    let mut gpioc = hal_peripherals.GPIOC.split(&mut rcc.ahb2);

    //Timer
    let mut timer = Delay::new(cortex_peripherals.SYST, clocks);

    //I2C
    let mut scl = gpioa
        .pa9
        .into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    scl.internal_pull_up(&mut gpioa.pupdr, true);
    let scl = scl.into_af4(&mut gpioa.moder, &mut gpioa.afrh);
    let mut sda = gpioa
        .pa10
        .into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    sda.internal_pull_up(&mut gpioa.pupdr, true);
    let sda = sda.into_af4(&mut gpioa.moder, &mut gpioa.afrh);
    let mut i2c = I2c::i2c1(
        hal_peripherals.I2C1,
        (scl, sda),
        100.khz(),
        clocks,
        &mut rcc.apb1r1,
    );

    let mut led_controller_enable_pin = gpioc
        .pc3
        .into_push_pull_output(&mut gpioc.moder, &mut gpioc.otyper);
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
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
