#![no_main]
#![no_std]

mod stm32f4xx;

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f4xx::system::system_init;

use stm32f4xx::peripherals::rccdef::RCC;

use stm32f4xx::peripherals::timdef::{TIM1, TIM2};

use stm32f4xx::drivers::gpio::*;
use crate::stm32f4xx::peripherals::gpiodef::{GPIOB, GPIOE};

const TIM_CFG: GpioConfig = GpioConfig {
    mode: ALTERNATE,
    otyper: PUSH_PULL,
    ospeedr: HIGH_SPEED,
    pupdr: NO_PULL_UP_DOWN,
    alt_func_sel: 1
};

#[entry]
fn main() -> ! {

    system_init();

    // using TIM1_CH1 PE9 AF1
    // TIM2_CH4 PB11 AF1

    // configure GPIOs as alternate
    let tim1_ch1 = new_gpio(GPIOE, 9);
    let tim2_ch4 = new_gpio(GPIOB, 11);
    tim1_ch1.configure(TIM_CFG);
    tim2_ch4.configure(TIM_CFG);

    // enable TIM1 clock in RCC APB2ENR register
    RCC.apb2enr.set_bits(1 << 0);
    // enable TIM2 clock in RCC APB1ENR register
    RCC.apb1enr.set_bits(1 << 0);

    TIM1.ccmr1.set_bits(6 << 4);
    TIM1.bdtr.set_bits(1 << 15);
    TIM1.ccer.set_bits(1 << 0);

    TIM2.ccmr2.set_bits(6 << 12);
    TIM2.ccer.set_bits(1 << 12);

    TIM1.psc.set(100);
    TIM1.arr.set(100);
    TIM1.ccr1.set(50);

    TIM2.psc.set(100);
    TIM2.arr.set(100);
    TIM2.ccr4.set(25);

    TIM1.cr1.set_bits(1 << 0);
    TIM2.cr1.set_bits(1 << 0);

    loop {

    }

}
