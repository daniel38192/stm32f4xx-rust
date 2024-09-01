use crate::stm32f4xx::core::register::Register;

const USART1_ADDRESS: u32 = 0x40011000;
const USART2_ADDRESS: u32 = 0x40004400;
const USART3_ADDRESS: u32 = 0x40004800;
const UART4_ADDRESS: u32 = 0x40004C00;
const UART5_ADDRESS: u32 = 0x40005000;
const USART6_ADDRESS: u32 = 0x40011400;
#[allow(dead_code)]
pub const USART1: UsartTypedef = UsartTypedef {
    sr: Register {addr: USART1_ADDRESS},
    dr: Register {addr: USART1_ADDRESS + 0x04},
    brr: Register {addr: USART1_ADDRESS + 0x08},
    cr1: Register {addr: USART1_ADDRESS + 0x0C},
    cr2: Register {addr: USART1_ADDRESS + 0x10},
    cr3: Register {addr: USART1_ADDRESS + 0x14},
    gtpr: Register {addr: USART1_ADDRESS + 0x18},
};
#[allow(dead_code)]
pub const USART2: UsartTypedef = UsartTypedef {
    sr: Register {addr: USART2_ADDRESS},
    dr: Register {addr: USART2_ADDRESS + 0x04},
    brr: Register {addr: USART2_ADDRESS + 0x08},
    cr1: Register {addr: USART2_ADDRESS + 0x0C},
    cr2: Register {addr: USART2_ADDRESS + 0x10},
    cr3: Register {addr: USART2_ADDRESS + 0x14},
    gtpr: Register {addr: USART2_ADDRESS + 0x18},
};
#[allow(dead_code)]
pub const USART3: UsartTypedef = UsartTypedef {
    sr: Register {addr: USART3_ADDRESS},
    dr: Register {addr: USART3_ADDRESS + 0x04},
    brr: Register {addr: USART3_ADDRESS + 0x08},
    cr1: Register {addr: USART3_ADDRESS + 0x0C},
    cr2: Register {addr: USART3_ADDRESS + 0x10},
    cr3: Register {addr: USART3_ADDRESS + 0x14},
    gtpr: Register {addr: USART3_ADDRESS + 0x18},
};
#[allow(dead_code)]
pub const UART4: UsartTypedef = UsartTypedef {
    sr: Register {addr: UART4_ADDRESS},
    dr: Register {addr: UART4_ADDRESS + 0x04},
    brr: Register {addr: UART4_ADDRESS + 0x08},
    cr1: Register {addr: UART4_ADDRESS + 0x0C},
    cr2: Register {addr: UART4_ADDRESS + 0x10},
    cr3: Register {addr: UART4_ADDRESS + 0x14},
    gtpr: Register {addr: UART4_ADDRESS + 0x18},
};
#[allow(dead_code)]
pub const UART5: UsartTypedef = UsartTypedef {
    sr: Register {addr: UART5_ADDRESS},
    dr: Register {addr: UART5_ADDRESS + 0x04},
    brr: Register {addr: UART5_ADDRESS + 0x08},
    cr1: Register {addr: UART5_ADDRESS + 0x0C},
    cr2: Register {addr: UART5_ADDRESS + 0x10},
    cr3: Register {addr: UART5_ADDRESS + 0x14},
    gtpr: Register {addr: UART5_ADDRESS + 0x18},
};
#[allow(dead_code)]
pub const USART6: UsartTypedef = UsartTypedef {
    sr: Register {addr: USART6_ADDRESS},
    dr: Register {addr: USART6_ADDRESS + 0x04},
    brr: Register {addr: USART6_ADDRESS + 0x08},
    cr1: Register {addr: USART6_ADDRESS + 0x0C},
    cr2: Register {addr: USART6_ADDRESS + 0x10},
    cr3: Register {addr: USART6_ADDRESS + 0x14},
    gtpr: Register {addr: USART6_ADDRESS + 0x18},
};
#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub struct UsartTypedef{
    pub sr: Register,   //0x00
    pub dr: Register,   //0x04
    pub brr: Register,  //0x08
    pub cr1: Register,  //0x0C
    pub cr2: Register,  //0x10
    pub cr3: Register,  //0x14
    pub gtpr: Register  //0x18
}