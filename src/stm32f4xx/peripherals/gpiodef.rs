use crate::stm32f4xx::core::register::Register;
const GPIOA_ADDRESS: u32 = 0x40020000;
const GPIOE_ADDRESS: u32 = 0x40021000;

pub const GPIOA: GpioTypedef = GpioTypedef {
    moder: Register {addr : GPIOA_ADDRESS},
    otyper: Register {addr : GPIOA_ADDRESS + 0x04},
    ospeedr: Register {addr : GPIOA_ADDRESS + 0x08},
    pupdr: Register {addr : GPIOA_ADDRESS + 0x0C},
    idr: Register {addr : GPIOA_ADDRESS + 0x10},
    odr: Register {addr : GPIOA_ADDRESS + 0x14},
    bsrr: Register {addr : GPIOA_ADDRESS + 0x18},
    lckr: Register {addr : GPIOA_ADDRESS + 0x1C},
    afrl: Register {addr : GPIOA_ADDRESS + 0x20},
    afrh: Register {addr : GPIOA_ADDRESS + 0x24},
};

pub const GPIOE: GpioTypedef = GpioTypedef {
    moder: Register {addr : GPIOE_ADDRESS},
    otyper: Register {addr : GPIOE_ADDRESS + 0x04},
    ospeedr: Register {addr : GPIOE_ADDRESS + 0x08},
    pupdr: Register {addr : GPIOE_ADDRESS + 0x0C},
    idr: Register {addr : GPIOE_ADDRESS + 0x10},
    odr: Register {addr : GPIOE_ADDRESS + 0x14},
    bsrr: Register {addr : GPIOE_ADDRESS + 0x18},
    lckr: Register {addr : GPIOE_ADDRESS + 0x1C},
    afrl: Register {addr : GPIOE_ADDRESS + 0x20},
    afrh: Register {addr : GPIOE_ADDRESS + 0x24},
};

#[derive(PartialEq, Clone)]
pub struct GpioTypedef {
    pub moder: Register,
    pub otyper: Register,
    pub ospeedr: Register,
    pub pupdr: Register,
    pub idr: Register,
    pub odr: Register,
    pub bsrr: Register,
    pub lckr: Register,
    pub afrl: Register,
    pub afrh: Register,
}
