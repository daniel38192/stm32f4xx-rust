use crate::stm32f4xx::core::register::Register;

const RCC_ADDRESS: u32 = 0x40023800;

// Reset and clock control for
// STM32F405xx/07xx
pub const RCC: RccTypedef = RccTypedef {
    cr: Register {addr: RCC_ADDRESS},
    pllcfgr: Register {addr: RCC_ADDRESS + 0x04},
    cfgr: Register {addr: RCC_ADDRESS + 0x08},
    cir: Register {addr: RCC_ADDRESS + 0x0C},
    ahb1rstr: Register {addr: RCC_ADDRESS + 0x10},
    ahb2rstr: Register {addr: RCC_ADDRESS + 0x14},
    ahb3rstr: Register {addr: RCC_ADDRESS + 0x18},
    apb1rstr: Register {addr: RCC_ADDRESS + 0x20},
    apb2rstr: Register {addr: RCC_ADDRESS + 0x24},
    ahb1enr: Register {addr: RCC_ADDRESS + 0x30},
    ahb2enr: Register {addr: RCC_ADDRESS + 0x34},
    ahb3enr: Register {addr: RCC_ADDRESS + 0x38},
    apb1enr: Register {addr: RCC_ADDRESS + 0x40},
    apb2enr: Register {addr: RCC_ADDRESS + 0x44},
    ahb1lpenr: Register {addr: RCC_ADDRESS + 0x50},
    ahb2lpenr: Register {addr: RCC_ADDRESS + 0x54},
    ahb3lpenr: Register {addr: RCC_ADDRESS + 0x58},
    apb1lpenr: Register {addr: RCC_ADDRESS + 0x60},
    apb2lpenr: Register {addr: RCC_ADDRESS + 0x64},
    bdcr: Register {addr: RCC_ADDRESS + 0x70},
    csr: Register {addr: RCC_ADDRESS + 0x74},
    sscgr: Register {addr: RCC_ADDRESS + 0x80},
    plli2scfgr: Register {addr: RCC_ADDRESS + 0x84},
};

pub struct RccTypedef {
    pub cr: Register, // RCC clock control register (RCC_CR) Address offset: 0x00
    pub pllcfgr: Register, // RCC PLL configuration register (RCC_PLLCFGR) Address offset: 0x04
    pub cfgr: Register, // RCC clock configuration register (RCC_CFGR) Address offset: 0x08
    pub cir: Register, // RCC clock interrupt register (RCC_CIR) Address offset: 0x0C
    pub ahb1rstr: Register, // RCC AHB1 peripheral reset register (RCC_AHB1RSTR) Address offset: 0x10
    pub ahb2rstr: Register, // RCC AHB2 peripheral reset register (RCC_AHB2RSTR) Address offset: 0x14
    pub ahb3rstr: Register, // RCC AHB3 peripheral reset register (RCC_AHB3RSTR) Address offset: 0x18
    pub apb1rstr: Register, // RCC APB1 peripheral reset register (RCC_APB1RSTR) Address offset: 0x20
    pub apb2rstr: Register, // RCC APB2 peripheral reset register (RCC_APB2RSTR) Address offset: 0x24
    pub ahb1enr: Register, // RCC AHB1 peripheral clock enable register (RCC_AHB1ENR) Address offset: 0x30
    pub ahb2enr: Register, // RCC AHB2 peripheral clock enable register (RCC_AHB2ENR) Address offset: 0x34
    pub ahb3enr: Register, // RCC AHB3 peripheral clock enable register (RCC_AHB3ENR) Address offset: 0x38
    pub apb1enr: Register, // RCC APB1 peripheral clock enable register (RCC_APB1ENR) Address offset: 0x40
    pub apb2enr: Register, // RCC APB2 peripheral clock enable register (RCC_APB2ENR) Address offset: 0x44
    pub ahb1lpenr: Register, // RCC AHB1 peripheral clock enable in low power mode register (RCC_AHB1LPENR) Address offset: 0x50
    pub ahb2lpenr: Register, // RCC AHB2 peripheral clock enable in low power mode register (RCC_AHB2LPENR) Address offset: 0x54
    pub ahb3lpenr: Register, // RCC AHB3 peripheral clock enable in low power mode register (RCC_AHB3LPENR) Address offset: 0x58
    pub apb1lpenr: Register, // RCC APB1 peripheral clock enable in low power mode register (RCC_APB1LPENR) Address offset: 0x60
    pub apb2lpenr: Register, // RCC APB2 peripheral clock enable in low power mode register (RCC_APB2LPENR) Address offset: 0x64
    pub bdcr: Register, // RCC Backup domain control register (RCC_BDCR) Address offset: 0x70
    pub csr: Register, // RCC clock control & status register (RCC_CSR) Address offset: 0x74
    pub sscgr: Register, // RCC spread spectrum clock generation register (RCC_SSCGR) Address offset: 0x80
    pub plli2scfgr: Register, // RCC PLLI2S configuration register (RCC_PLLI2SCFGR) Address offset: 0x84
}