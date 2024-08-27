use crate::stm32f4xx::core::register::Register;

const GPIOA_ADDRESS: u32 = 0x40020000;
const GPIOB_ADDRESS: u32 = 0x40020400;
const GPIOC_ADDRESS: u32 = 0x40020800;
const GPIOD_ADDRESS: u32 = 0x40020C00;
const GPIOE_ADDRESS: u32 = 0x40021000;
const GPIOF_ADDRESS: u32 = 0x40021400;
const GPIOG_ADDRESS: u32 = 0x40021800;
const GPIOH_ADDRESS: u32 = 0x40021C00;
const GPIOI_ADDRESS: u32 = 0x40022000;

#[allow(dead_code)]
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
#[allow(dead_code)]
pub const GPIOB: GpioTypedef = GpioTypedef {
    moder: Register {addr : GPIOB_ADDRESS},
    otyper: Register {addr : GPIOB_ADDRESS + 0x04},
    ospeedr: Register {addr : GPIOB_ADDRESS + 0x08},
    pupdr: Register {addr : GPIOB_ADDRESS + 0x0C},
    idr: Register {addr : GPIOB_ADDRESS + 0x10},
    odr: Register {addr : GPIOB_ADDRESS + 0x14},
    bsrr: Register {addr : GPIOB_ADDRESS + 0x18},
    lckr: Register {addr : GPIOB_ADDRESS + 0x1C},
    afrl: Register {addr : GPIOB_ADDRESS + 0x20},
    afrh: Register {addr : GPIOB_ADDRESS + 0x24},
};
#[allow(dead_code)]
pub const GPIOC: GpioTypedef = GpioTypedef {
    moder: Register {addr : GPIOC_ADDRESS},
    otyper: Register {addr : GPIOC_ADDRESS + 0x04},
    ospeedr: Register {addr : GPIOC_ADDRESS + 0x08},
    pupdr: Register {addr : GPIOC_ADDRESS + 0x0C},
    idr: Register {addr : GPIOC_ADDRESS + 0x10},
    odr: Register {addr : GPIOC_ADDRESS + 0x14},
    bsrr: Register {addr : GPIOC_ADDRESS + 0x18},
    lckr: Register {addr : GPIOC_ADDRESS + 0x1C},
    afrl: Register {addr : GPIOC_ADDRESS + 0x20},
    afrh: Register {addr : GPIOC_ADDRESS + 0x24},
};
#[allow(dead_code)]
pub const GPIOD: GpioTypedef = GpioTypedef {
    moder: Register {addr : GPIOD_ADDRESS},
    otyper: Register {addr : GPIOD_ADDRESS + 0x04},
    ospeedr: Register {addr : GPIOD_ADDRESS + 0x08},
    pupdr: Register {addr : GPIOD_ADDRESS + 0x0C},
    idr: Register {addr : GPIOD_ADDRESS + 0x10},
    odr: Register {addr : GPIOD_ADDRESS + 0x14},
    bsrr: Register {addr : GPIOD_ADDRESS + 0x18},
    lckr: Register {addr : GPIOD_ADDRESS + 0x1C},
    afrl: Register {addr : GPIOD_ADDRESS + 0x20},
    afrh: Register {addr : GPIOD_ADDRESS + 0x24},
};
#[allow(dead_code)]
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
#[allow(dead_code)]
pub const GPIOF: GpioTypedef = GpioTypedef {
    moder: Register {addr : GPIOF_ADDRESS},
    otyper: Register {addr : GPIOF_ADDRESS + 0x04},
    ospeedr: Register {addr : GPIOF_ADDRESS + 0x08},
    pupdr: Register {addr : GPIOF_ADDRESS + 0x0C},
    idr: Register {addr : GPIOF_ADDRESS + 0x10},
    odr: Register {addr : GPIOF_ADDRESS + 0x14},
    bsrr: Register {addr : GPIOF_ADDRESS + 0x18},
    lckr: Register {addr : GPIOF_ADDRESS + 0x1C},
    afrl: Register {addr : GPIOF_ADDRESS + 0x20},
    afrh: Register {addr : GPIOF_ADDRESS + 0x24},
};
#[allow(dead_code)]
pub const GPIOG: GpioTypedef = GpioTypedef {
    moder: Register {addr : GPIOG_ADDRESS},
    otyper: Register {addr : GPIOG_ADDRESS + 0x04},
    ospeedr: Register {addr : GPIOG_ADDRESS + 0x08},
    pupdr: Register {addr : GPIOG_ADDRESS + 0x0C},
    idr: Register {addr : GPIOG_ADDRESS + 0x10},
    odr: Register {addr : GPIOG_ADDRESS + 0x14},
    bsrr: Register {addr : GPIOG_ADDRESS + 0x18},
    lckr: Register {addr : GPIOG_ADDRESS + 0x1C},
    afrl: Register {addr : GPIOG_ADDRESS + 0x20},
    afrh: Register {addr : GPIOG_ADDRESS + 0x24},
};
#[allow(dead_code)]
pub const GPIOH: GpioTypedef = GpioTypedef {
    moder: Register {addr : GPIOH_ADDRESS},
    otyper: Register {addr : GPIOH_ADDRESS + 0x04},
    ospeedr: Register {addr : GPIOH_ADDRESS + 0x08},
    pupdr: Register {addr : GPIOH_ADDRESS + 0x0C},
    idr: Register {addr : GPIOH_ADDRESS + 0x10},
    odr: Register {addr : GPIOH_ADDRESS + 0x14},
    bsrr: Register {addr : GPIOH_ADDRESS + 0x18},
    lckr: Register {addr : GPIOH_ADDRESS + 0x1C},
    afrl: Register {addr : GPIOH_ADDRESS + 0x20},
    afrh: Register {addr : GPIOH_ADDRESS + 0x24},
};
#[allow(dead_code)]
pub const GPIOI: GpioTypedef = GpioTypedef {
    moder: Register {addr : GPIOI_ADDRESS},
    otyper: Register {addr : GPIOI_ADDRESS + 0x04},
    ospeedr: Register {addr : GPIOI_ADDRESS + 0x08},
    pupdr: Register {addr : GPIOI_ADDRESS + 0x0C},
    idr: Register {addr : GPIOI_ADDRESS + 0x10},
    odr: Register {addr : GPIOI_ADDRESS + 0x14},
    bsrr: Register {addr : GPIOI_ADDRESS + 0x18},
    lckr: Register {addr : GPIOI_ADDRESS + 0x1C},
    afrl: Register {addr : GPIOI_ADDRESS + 0x20},
    afrh: Register {addr : GPIOI_ADDRESS + 0x24},
};
#[allow(dead_code)]
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
