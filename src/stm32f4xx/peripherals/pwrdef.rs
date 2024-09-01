use crate::stm32f4xx::core::register::Register;

const PWR_ADDRESS: u32 = 0x40007000;

#[allow(dead_code)]
pub const PWR: PwrTypeDef = PwrTypeDef {
    cr: Register {addr: PWR_ADDRESS},
    csr: Register {addr: PWR_ADDRESS}
};

#[allow(dead_code)]
pub struct PwrTypeDef {
    pub cr: Register,
    pub csr: Register,
}