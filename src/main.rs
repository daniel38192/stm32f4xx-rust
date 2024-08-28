#![no_main]
#![no_std]

mod stm32f4xx;

extern crate panic_halt;
use cortex_m_rt::entry;
use stm32f4xx::system::system_init;

use stm32f4xx::drivers::gpio::*;
use stm32f4xx::drivers::lcd_hd44780::Lcd8bit;
use crate::stm32f4xx::peripherals::gpiodef::{GPIOB, GPIOE};

use crate::stm32f4xx::core::utils::i32_to_hex_str;

#[entry]
fn main() -> ! {

    system_init();

    let lcd1 = Lcd8bit {
        register_select: Gpio {port: GPIOE, pin_number: 10},
        read_write: Gpio {port: GPIOE, pin_number: 12},
        enable: Gpio {port: GPIOE, pin_number: 11},

        d0: Gpio {port: GPIOB, pin_number: 13},
        d1: Gpio {port: GPIOB, pin_number: 11},
        d2: Gpio {port: GPIOE, pin_number: 15},
        d3: Gpio {port: GPIOE, pin_number: 13},

        d4: Gpio {port: GPIOB, pin_number: 14},
        d5: Gpio {port: GPIOB, pin_number: 12},
        d6: Gpio {port: GPIOB, pin_number: 10},
        d7: Gpio {port: GPIOE, pin_number: 14},
    };

    lcd1.configure();

    lcd1.print("hello, world!");

    lcd1.set_cursor(1, 2);

    lcd1.print("rust stm32");

    lcd1.set_cursor(12, 2);


    lcd1.print(i32_to_hex_str(255).as_str());

    loop {

    }
}
