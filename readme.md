# Table of Contents

1.  [Hello world project to learn about embedded development using the STM32F411 CE (black pill) microcontroller](#org7fce06e)
    1.  [Flash and run](#orgad01094)



<a id="org7fce06e"></a>

# Hello world project to learn about embedded development using the STM32F411 CE (black pill) microcontroller


<a id="orgad01094"></a>

## Build and flash:
    
        cargo build --release
        probe-rs run --chip STM32F411CE --protocol SWD target/thumbv7em-none-eabihf/release/black_pill_hw

