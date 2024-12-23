#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4::stm32f411;

#[entry]
fn main() -> ! {
    // Take ownership of the device peripherals
    let dp = stm32f411::Peripherals::take().unwrap();

    // Enable GPIOA clock in the RCC (Reset and Clock Control) register
    let rcc = dp.RCC;
    rcc.ahb1enr.modify(|_, w| w.gpioaen().set_bit());

    // Configure PA5 as a push-pull output
    let gpioa = dp.GPIOA;
    gpioa.moder.modify(|_, w| w.moder5().output());
    gpioa.otyper.modify(|_, w| w.ot5().push_pull());
    gpioa.ospeedr.modify(|_, w| w.ospeedr5().low_speed());

    loop {
        // Toggle the LED
        gpioa.odr.modify(|r, w| w.odr5().bit(!r.odr5().bit()));

        // Delay (basic busy-loop delay)
        for _ in 0..1_000_000 {
            cortex_m::asm::nop();
        }
    }
}
