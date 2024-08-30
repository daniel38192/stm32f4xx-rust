use super::peripherals::rccdef::RCC;
use crate::stm32f4xx::core::delay::tim6_delay_init;

pub fn system_init(){
    system_clock_config();
    tim6_delay_init();
}

fn system_clock_config(){
    // enable HSI

    RCC.cr.set_bits(1 << 0);

    // test
    // wait for HSI RDY flag

    while!RCC.cr.has_bits(1 << 1){

    }

}