#![no_main]
#![no_std]

use cortex_m_rt::entry;
use fugit::HertzU32;
use panic_halt as _;
use stm32f4xx_hal::{
    i2c::{I2c, Mode},
    pac,
    prelude::*,
};

#[entry]
fn main() -> ! {
    // Access STM32 peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Split GPIOB into individual pins
    let gpiob = dp.GPIOB.split();

    // Configure PB6 (SCL) and PB7 (SDA) for I2C1
    let scl = gpiob.pb6.into_alternate().set_open_drain();
    let sda = gpiob.pb7.into_alternate().set_open_drain();

    // Enable the clock and configure the system
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(); // Default 16 MHz clock (HSI)

    // Configure I2C1 with the desired speed (e.g., 100 kHz)
    let _i2c = I2c::new(
        dp.I2C1,
        (scl, sda),
        Mode::Standard {
            frequency: HertzU32::from_raw(100_000),
        }, // Set the mode to 100 kHz standard mode
        &clocks,
    );
    // Use I2C to communicate with a peripheral (e.g., SSD1306)
    // Example: Sending data to an I2C device at address 0x3C
    //let address = 0x3C; // Replace with your device's I2C address
    //let data = [0x00, 0xAF]; // Example data: SSD1306 "Display ON" command
    //i2c.write(address, &data).unwrap();

    loop {
        // Main loop
    }
}
