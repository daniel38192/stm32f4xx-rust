#![no_main]
#![no_std]

mod stm32f4xx;

extern crate panic_halt;

use cortex_m_rt::entry;

use stm32f4xx::system::system_init;

use stm32f4xx::peripherals::gpiodef::{GPIOA, GPIOE};

use stm32f4xx::drivers::gpio::*;

const LEDCONFIG: GpioConfig = GpioConfig {
    mode: OUTPUT,
    otyper: PUSH_PULL,
    ospeedr: LOW_SPEED,
    pupdr: NO_PULL_UP_DOWN,
    alt_func_sel:0
};

const BUTTONCONFIG: GpioConfig = GpioConfig {
    mode: INPUT,
    otyper: PUSH_PULL,
    ospeedr: LOW_SPEED,
    pupdr: PULL_UP,
    alt_func_sel:0
};

#[entry]
fn main()  -> ! {

    system_init();


    let led1 = new_gpio(GPIOA, 7);
    let led2 = new_gpio(GPIOA, 6);
    let button1 = new_gpio(GPIOE, 3);
    let button2 = new_gpio(GPIOE, 4);

    led1.configure(LEDCONFIG);
    led2.configure(LEDCONFIG);
    button1.configure(BUTTONCONFIG);
    button2.configure(BUTTONCONFIG);

    loop {
        if button1.get() {
            led1.high()
        } else {
            led1.low()
        }

        if button2.get() {
            led2.high()
        } else {
            led2.low()
        }
    }

}
