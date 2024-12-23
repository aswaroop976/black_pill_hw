// simple blinky program, gonna try and interface with display now cya

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _; // Panic handler
use stm32f4::stm32f411;

#[entry]
fn main() -> ! {
    // Get access to the device peripherals
    let dp = stm32f411::Peripherals::take().unwrap();

    // Enable GPIOC clock in RCC
    let rcc = dp.RCC;
    rcc.ahb1enr.modify(|_, w| w.gpiocen().set_bit());

    // Configure PC13 as push-pull output
    let gpioc = dp.GPIOC;
    gpioc.moder.modify(|_, w| w.moder13().output()); // Set PC13 as output
    gpioc.otyper.modify(|_, w| w.ot13().push_pull()); // Set PC13 as push-pull
    gpioc.ospeedr.modify(|_, w| w.ospeedr13().low_speed()); // Set low speed

    loop {
        // Turn the LED on (set PC13 high)
        gpioc.odr.modify(|_, w| w.odr13().set_bit());
        cortex_m::asm::delay(8_000_000); // Delay

        // Turn the LED off (set PC13 low)
        gpioc.odr.modify(|_, w| w.odr13().clear_bit());
        cortex_m::asm::delay(8_000_000); // Delay
    }
}
