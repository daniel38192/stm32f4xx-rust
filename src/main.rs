#![no_main]
#![no_std]

mod stm32f4xx;

extern crate panic_halt;

use cortex_m_rt::entry;

use stm32f4xx::system::system_init;

use stm32f4xx::peripherals::gpiodef::{GPIOA, GPIOE};

use stm32f4xx::drivers::gpio::*;


#[entry]
fn main()  -> ! {

    system_init();

    //RCC.ahb1enr.set_bits(1 << 0);

    //GPIOA.moder.set_bits(1 << 14);
    //GPIOA.moder.set_bits(1 << 12);

    //GPIOA.bsrr.set_bits(1 << 6);

    let led1 = new_gpio(GPIOA, 7);
    let led2 = new_gpio(GPIOA, 6);

    led1.configure(GpioConfig {mode: OUTPUT, otyper: PUSH_PULL, ospeedr: LOW_SPEED, pupdr: NO_PULL_UP_DOWN, alt_func_sel:0});
    led2.configure(GpioConfig {mode: OUTPUT, otyper: PUSH_PULL, ospeedr: LOW_SPEED, pupdr: NO_PULL_UP_DOWN, alt_func_sel:0});

    let button1 = new_gpio(GPIOE, 3);
    let button2 = new_gpio(GPIOE, 4);

    button1.configure(GpioConfig {mode: INPUT, otyper: PUSH_PULL, ospeedr: LOW_SPEED, pupdr: PULL_UP, alt_func_sel:0});
    button2.configure(GpioConfig {mode: INPUT, otyper: PUSH_PULL, ospeedr: LOW_SPEED, pupdr: PULL_UP, alt_func_sel:0});

    led1.high();
    led2.high();

    loop {
        if button1.get() {
            led1.low()
        } else {
            led1.high()
        }

        if button2.get() {
            led2.low()
        } else {
            led2.high()
        }
    }
}
