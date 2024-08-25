use super::peripherals::rccdef::RCC;
pub fn system_init(){
    system_clock_config()
}

fn system_clock_config(){
    // enable HSI

    RCC.cr.set_bits(1 << 0);

    // test
    // wait for HSI RDY flag

    while!RCC.cr.has_bits(1 << 1){

    }
}