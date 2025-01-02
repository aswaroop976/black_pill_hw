#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use fugit::HertzU32;
use panic_halt as _;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
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
    let i2c = I2c::new(
        dp.I2C1,
        (scl, sda),
        Mode::Standard {
            frequency: HertzU32::from_raw(100_000),
        }, // Set the mode to 100 kHz standard mode
        &clocks,
    );

    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();
    // Use I2C to communicate with a peripheral (e.g., SSD1306)
    // Example: Sending data to an I2C device at address 0x3C
    //let address = 0x3C; // Replace with your device's I2C address
    //let data = [0x00, 0xAF]; // Example data: SSD1306 "Display ON" command
    //i2c.write(address, &data).unwrap();

    let gpioc = dp.GPIOC;
    gpioc.moder().modify(|_, w| w.moder13().output()); // Set PC13 as output
    gpioc.otyper().modify(|_, w| w.ot13().push_pull()); // Set PC13 as push-pull
    gpioc.ospeedr().modify(|_, w| w.ospeedr13().low_speed()); // Set low speed

    loop {
        // Turn the LED on (set PC13 high)
        gpioc.odr().modify(|_, w| w.odr13().set_bit());
        cortex_m::asm::delay(8_000_000); // Delay

        // Turn the LED off (set PC13 low)
        gpioc.odr().modify(|_, w| w.odr13().clear_bit());
        cortex_m::asm::delay(8_000_000); // Delay
    }
}
