use crate::stm32f4xx::peripherals::rccdef::RCC;
use crate::stm32f4xx::peripherals::timdef::TIM6;

//basic delay function, useful for test
#[allow(dead_code)]
pub fn non_exact_time_delay(delay: u32){
    let mut count = 0;
    while count < delay {
        count+=1;
    }
//some applications may require exact time delay
}

#[allow(dead_code)]
pub fn delay(milliseconds: u32){
    for _ in 0..milliseconds {
        tim6_delay(1000)
    }
}
#[allow(dead_code)]
pub fn delay_microseconds(microseconds: u32){
    for _ in 0..microseconds {
        tim6_delay(1);
    }
}

fn tim6_delay(delay: u32) {
    TIM6.cnt.set(0);
    while TIM6.cnt.get() < delay {

    }
}

pub fn tim6_delay_init(){
    // enable TIM6 clock in RCC APB2ENR register
    RCC.apb1enr.set_bits(1 << 4);

    TIM6.psc.set(16);
    TIM6.cr1.set_bits(1 << 0);
}
