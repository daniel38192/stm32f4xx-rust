#![no_main]
#![no_std]

mod stm32f4xx;

extern crate panic_halt;
use cortex_m_rt::entry;
use stm32f4xx::system::system_init;

use stm32f4xx::drivers::gpio::*;
use stm32f4xx::drivers::lcd_hd44780::Lcd8bit;
use crate::stm32f4xx::core::delay::delay;
use crate::stm32f4xx::core::utils::i32_to_str;
use crate::stm32f4xx::drivers::serial::Serial;
use crate::stm32f4xx::peripherals::gpiodef::{GPIOA, GPIOB, GPIOE};
use crate::stm32f4xx::peripherals::usartdef::USART1;

const LCD1: Lcd8bit = Lcd8bit {
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

#[entry]
fn main() -> ! {

    system_init();

    LCD1.configure();

    let serial1 = Serial {
        serial_port: USART1,
        gpio_port: GPIOA,
        baudrate: 115200,
        tx_pin: 9,
        rx_pin: 10,
        alt_func_selector: 7,
    };

    serial1.configure();

    serial1.println("Hello, World!");
    serial1.println("STM32 Rust Embedded");
    serial1.println("USART1");

    LCD1.print(i32_to_str(GPIOA.afrh.get() as i32).as_str());
    LCD1.set_cursor(1 , 2);
    LCD1.print(i32_to_str(USART1.cr1.get() as i32).as_str());

    loop {

        let mut received_data = serial1.read_byte();

        if received_data != 0 {
            serial1.print("Received: "); serial1.send_byte(received_data); serial1.new_line()
        }

        delay(8);

    }

}
