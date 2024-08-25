use crate::stm32f4xx::peripherals::rccdef::RCC;
use crate::stm32f4xx::peripherals::gpiodef::*;
/* Gpio modes */
#[allow(dead_code)]
pub const INPUT: u32 = 0;
#[allow(dead_code)]
pub const OUTPUT: u32 = 1;
#[allow(dead_code)]
pub const ALTERNATE: u32 = 2;
#[allow(dead_code)]
pub const ANALOG: u32 = 3;

/* output type */
#[allow(dead_code)]
pub const PUSH_PULL: u32 = 0;
#[allow(dead_code)]
pub const OPEN_DRAIN: u32 = 1;

/* output speed */
#[allow(dead_code)]
pub const LOW_SPEED: u32 = 0;
#[allow(dead_code)]
pub const MEDIUM_SPEED: u32 = 1;
#[allow(dead_code)]
pub const HIGH_SPEED: u32 = 2;
#[allow(dead_code)]
pub const VERY_HIGH_SPEED: u32 = 3;

/* pull up / pull down*/
#[allow(dead_code)]
pub const NO_PULL_UP_DOWN: u32 = 0;
#[allow(dead_code)]
pub const PULL_UP: u32 = 1;
#[allow(dead_code)]
pub const PULL_DOWN: u32 = 2;

#[allow(dead_code)]
pub struct Gpio {
    pin_number: u32,
    port: GpioTypedef
}
#[allow(dead_code)]
pub struct GpioConfig {
    pub mode: u32,
    pub otyper: u32,
    pub ospeedr: u32,
    pub pupdr: u32,
    pub alt_func_sel: u32
}
#[allow(dead_code)]
pub fn new_gpio(port: GpioTypedef, pin_number: u32) -> Gpio{
    Gpio {
        pin_number,
        port,
    }
}

impl Gpio {
    #[allow(dead_code)]
    pub fn high(&self){
        self.set(true)
    }
    #[allow(dead_code)]
    pub fn low(&self){
        self.set(false)
    }
    #[allow(dead_code)]
    pub fn set(&self, value: bool){
        if value {
            self.port.bsrr.set_bits(1 << self.pin_number)
        } else {
            self.port.bsrr.set_bits(1 << self.pin_number + 16)
        }
    }
    #[allow(dead_code)]
    pub fn get(&self) -> bool {
        self.port.idr.has_bits(1 << self.pin_number)
    }
    #[allow(dead_code)]
    pub fn configure(&self, gpio_config: GpioConfig){

        self.enable_rcc();

        self.port.moder.set_bits(gpio_config.mode << self.pin_number * 2);
        self.port.otyper.set_bits(gpio_config.otyper << self.pin_number * 2);
        self.port.ospeedr.set_bits(gpio_config.ospeedr << self.pin_number * 2);
        self.port.pupdr.set_bits(gpio_config.pupdr << self.pin_number * 2);

        if gpio_config.mode == ALTERNATE {
            if self.pin_number > 7 {
                self.port.afrh.set_bits(gpio_config.alt_func_sel << ((self.pin_number * 4) - 32));
            } else {
                self.port.afrl.set_bits(gpio_config.alt_func_sel << (self.pin_number * 4));
            }
        }

    }

    fn enable_rcc(&self){

        match self.port {
            GPIOA => RCC.ahb1enr.set_bits(1 << 0),
            GPIOB => RCC.ahb1enr.set_bits(1 << 1),
            GPIOC => RCC.ahb1enr.set_bits(1 << 2),
            GPIOD => RCC.ahb1enr.set_bits(1 << 3),
            GPIOE => RCC.ahb1enr.set_bits(1 << 4),
            GPIOF => RCC.ahb1enr.set_bits(1 << 5),
            GPIOG => RCC.ahb1enr.set_bits(1 << 6),
            GPIOH => RCC.ahb1enr.set_bits(1 << 7),
            GPIOI => RCC.ahb1enr.set_bits(1 << 8),
            _ => {},
        }

    }

}
