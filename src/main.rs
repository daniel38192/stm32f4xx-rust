#![no_main]
#![no_std]

mod stm32f4xx;
mod memory_alloc;

extern crate panic_halt;

extern crate alloc;

use cortex_m_rt::entry;

use crate::stm32f4xx::system::system_init;
use crate::memory_alloc::memory_allocator_init;
use crate::stm32f4xx::core::delay::delay;
use crate::stm32f4xx::drivers::serial::Serial;
use crate::stm32f4xx::peripherals::gpiodef::GPIOA;
use crate::stm32f4xx::peripherals::usartdef::USART1;
use crate::stm32f4xx::drivers::real_time_clock::{rtc_configure_once, Date};

const SERIAL1: Serial = Serial {
    serial_port: USART1,
    gpio_port: GPIOA,
    baudrate: 115200,
    tx_pin: 9,
    rx_pin: 10,
    alt_func_selector: 7,
};

#[entry]
fn main() -> ! {

    system_init();
    memory_allocator_init();

    SERIAL1.configure();

    SERIAL1.println("RTC test");
    SERIAL1.println("STM32 Rust Embedded");
    SERIAL1.println("USART1");

    rtc_configure_once();

    loop {


        SERIAL1.println(Date::now().full_date().as_str());
        SERIAL1.println(Date::now().date_only().as_str());
        SERIAL1.println(Date::now().time_only().as_str());
        SERIAL1.println(Date::now().time_type_kitchen().as_str());
        delay(1000);


    }

}


