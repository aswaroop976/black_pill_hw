#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Rectangle, Triangle},
    style::PrimitiveStyle,
};
use panic_halt as _; // Panic handler
use ssd1306::{prelude::*, Builder};
use stm32f4::stm32f411;

#[entry]
fn main() -> ! {
    // Take ownership of the STM32F411 peripherals
    let dp = stm32f411::Peripherals::take().unwrap();

    // Enable GPIOB clock for I2C
    let rcc = dp.RCC;
    rcc.ahb1enr.modify(|_, w| w.gpioben().set_bit());

    // Configure PB6 (SCL) and PB7 (SDA) as alternate function (AF4 for I2C1)
    let gpiob = dp.GPIOB;
    gpiob.moder.modify(|_, w| {
        w.moder6().alternate();
        w.moder7().alternate()
    });
    gpiob.otyper.modify(|_, w| {
        w.ot6().open_drain();
        w.ot7().open_drain()
    });
    gpiob.ospeedr.modify(|_, w| {
        w.ospeedr6().high_speed();
        w.ospeedr7().high_speed()
    });
    gpiob.pupdr.modify(|_, w| {
        w.pupdr6().pull_up();
        w.pupdr7().pull_up()
    });
    gpiob.afrl.modify(|_, w| {
        w.afrl6().af4();
        w.afrl7().af4()
    });

    // Enable I2C1 clock
    rcc.apb1enr.modify(|_, w| w.i2c1en().set_bit());

    // Configure I2C1
    let i2c1 = dp.I2C1;
    i2c1.cr2.modify(|_, w| w.freq().bits(42)); // APB1 clock frequency in MHz
    i2c1.ccr
        .modify(|_, w| w.f_s().fast_mode().duty().duty_2().bits(35)); // Fast mode (400 kHz)
    i2c1.trise.modify(|_, w| w.trise().bits(14)); // Maximum rise time
    i2c1.cr1.modify(|_, w| w.pe().set_bit()); // Enable I2C1

    // Initialize SSD1306 using the `ssd1306` crate
    let interface = ssd1306::I2CDisplayInterface::new(i2c1);
    let mut display: GraphicsMode<_> = Builder::new()
        .size(DisplaySize128x64)
        .connect(interface)
        .into();

    display.init().unwrap();
    display.flush().unwrap();

    // Draw graphics on the display
    let rectangle = Rectangle::new(Point::new(0, 0), Size::new(64, 32))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));
    let triangle = Triangle::new(Point::new(64, 32), Point::new(128, 32), Point::new(96, 0))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));

    display.draw(&rectangle).unwrap();
    display.draw(&triangle).unwrap();
    display.flush().unwrap();

    loop {}
}

// blinky program ==============================================================
//fn main() -> ! {
//    // Get access to the device peripherals
//    let dp = stm32f411::Peripherals::take().unwrap();
//
//    // Enable GPIOC clock in RCC
//    let rcc = dp.RCC;
//    rcc.ahb1enr.modify(|_, w| w.gpiocen().set_bit());
//
//    // Configure PC13 as push-pull output
//    let gpioc = dp.GPIOC;
//    gpioc.moder.modify(|_, w| w.moder13().output()); // Set PC13 as output
//    gpioc.otyper.modify(|_, w| w.ot13().push_pull()); // Set PC13 as push-pull
//    gpioc.ospeedr.modify(|_, w| w.ospeedr13().low_speed()); // Set low speed
//
//    loop {
//        // Turn the LED on (set PC13 high)
//        gpioc.odr.modify(|_, w| w.odr13().set_bit());
//        cortex_m::asm::delay(8_000_000); // Delay
//
//        // Turn the LED off (set PC13 low)
//        gpioc.odr.modify(|_, w| w.odr13().clear_bit());
//        cortex_m::asm::delay(8_000_000); // Delay
//    }
//}

