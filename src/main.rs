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
use stm32f4xx_hal::{i2c::I2c, pac, prelude::*};

#[entry]
fn main() -> ! {
    // Get access to the device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // Set up the system clock
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();

    // Configure GPIO pins for I2C1 (PB6 = SCL, PB7 = SDA)
    let gpiob = dp.GPIOB.split();
    let scl = gpiob.pb6.into_alternate_af4().set_open_drain();
    let sda = gpiob.pb7.into_alternate_af4().set_open_drain();

    // Initialize the I2C1 interface
    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks);

    // Initialize the SSD1306 display
    let interface = ssd1306::I2CDisplayInterface::new(i2c);
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

