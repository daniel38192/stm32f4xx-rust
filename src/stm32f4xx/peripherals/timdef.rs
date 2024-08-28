
use crate::stm32f4xx::core::register::Register;
use crate::stm32f4xx::core::register::EMPTY_REGISTER;

const TIM1_ADDRESS: u32 = 0x40010000;
const TIM2_ADDRESS: u32 = 0x40000000;
const TIM3_ADDRESS: u32 = 0x40000400;
const TIM4_ADDRESS: u32 = 0x40000800;
const TIM5_ADDRESS: u32 = 0x40000C00;
const TIM6_ADDRESS: u32 = 0x40001000;
const TIM7_ADDRESS: u32 = 0x40001400;
const TIM8_ADDRESS: u32 = 0x40010400;
const TIM9_ADDRESS: u32 = 0x40014000;
const TIM10_ADDRESS: u32 = 0x40014400;
const TIM11_ADDRESS: u32 = 0x40014800;
const TIM12_ADDRESS: u32 = 0x40001800;
const TIM13_ADDRESS: u32 = 0x40001C00;
const TIM14_ADDRESS: u32 = 0x40002000;

#[allow(dead_code)]
pub const TIM1: TimTypedef = TimTypedef {
    cr1: Register{addr:TIM1_ADDRESS},
    cr2: Register{addr:TIM1_ADDRESS + 0x04},
    smcr: Register { addr: TIM1_ADDRESS + 0x08 },
    dier: Register{addr:TIM1_ADDRESS + 0x0C},
    sr: Register{addr:TIM1_ADDRESS + 0x10},
    egr: Register{addr:TIM1_ADDRESS + 0x14},
    ccmr1: Register { addr: TIM1_ADDRESS + 0x18 },
    ccmr2: Register { addr: TIM1_ADDRESS + 0x1C },
    ccer: Register { addr: TIM1_ADDRESS + 0x20 },
    cnt: Register{addr:TIM1_ADDRESS + 0x24},
    psc: Register{addr:TIM1_ADDRESS + 0x28},
    arr: Register{addr:TIM1_ADDRESS + 0x2C},
    rcr: Register { addr: TIM1_ADDRESS + 0x30 },
    ccr1: Register { addr: TIM1_ADDRESS + 0x34 },
    ccr2: Register { addr: TIM1_ADDRESS + 0x38 },
    ccr3: Register { addr: TIM1_ADDRESS + 0x3C },
    ccr4: Register { addr: TIM1_ADDRESS + 0x40 },
    bdtr: Register { addr: TIM1_ADDRESS + 0x44 },
    dcr: Register { addr: TIM1_ADDRESS + 0x48 },
    dmar: Register { addr: TIM1_ADDRESS + 0x4C },
    or: EMPTY_REGISTER,
};

#[allow(dead_code)]
pub const TIM2: TimTypedef = TimTypedef {
    cr1: Register { addr: TIM2_ADDRESS },
    cr2: Register { addr: TIM2_ADDRESS + 0x04 },
    smcr: Register { addr: TIM2_ADDRESS + 0x08 },
    dier: Register { addr: TIM2_ADDRESS + 0x0C },
    sr: Register { addr: TIM2_ADDRESS + 0x10 },
    egr: Register { addr: TIM2_ADDRESS + 0x14 },
    ccmr1: Register { addr: TIM2_ADDRESS + 0x18 },
    ccmr2: Register { addr: TIM2_ADDRESS + 0x1C },
    ccer: Register { addr: TIM2_ADDRESS + 0x20 },
    cnt: Register { addr: TIM2_ADDRESS + 0x24 },
    psc: Register { addr: TIM2_ADDRESS + 0x28 },
    arr: Register { addr: TIM2_ADDRESS + 0x2C },
    rcr: EMPTY_REGISTER,
    ccr1: Register { addr: TIM2_ADDRESS + 0x34 },
    ccr2: Register { addr: TIM2_ADDRESS + 0x38 },
    ccr3: Register { addr: TIM2_ADDRESS + 0x3C },
    ccr4: Register { addr: TIM2_ADDRESS + 0x40 },
    bdtr: EMPTY_REGISTER,
    dcr: Register { addr: TIM2_ADDRESS + 0x48 },
    dmar: Register { addr: TIM2_ADDRESS + 0x4C },
    or: Register { addr: TIM2_ADDRESS + 0x50 },
};

#[allow(dead_code)]
pub const TIM3: TimTypedef = TimTypedef {
    cr1: Register { addr: TIM3_ADDRESS },
    cr2: Register { addr: TIM3_ADDRESS + 0x04 },
    smcr: Register { addr: TIM3_ADDRESS + 0x08 },
    dier: Register { addr: TIM3_ADDRESS + 0x0C },
    sr: Register { addr: TIM3_ADDRESS + 0x10 },
    egr: Register { addr: TIM3_ADDRESS + 0x14 },
    ccmr1: Register { addr: TIM3_ADDRESS + 0x18 },
    ccmr2: Register { addr: TIM3_ADDRESS + 0x1C },
    ccer: Register { addr: TIM3_ADDRESS + 0x20 },
    cnt: Register { addr: TIM3_ADDRESS + 0x24 },
    psc: Register { addr: TIM3_ADDRESS + 0x28 },
    arr: Register { addr: TIM3_ADDRESS + 0x2C },
    rcr: EMPTY_REGISTER,
    ccr1: Register { addr: TIM3_ADDRESS + 0x34 },
    ccr2: Register { addr: TIM3_ADDRESS + 0x38 },
    ccr3: Register { addr: TIM3_ADDRESS + 0x3C },
    ccr4: Register { addr: TIM3_ADDRESS + 0x40 },
    bdtr: EMPTY_REGISTER,
    dcr: Register { addr: TIM3_ADDRESS + 0x48 },
    dmar: Register { addr: TIM3_ADDRESS + 0x4C },
    or: EMPTY_REGISTER,
};

#[allow(dead_code)]
pub const TIM4: TimTypedef = TimTypedef {
    cr1: Register { addr: TIM4_ADDRESS },
    cr2: Register { addr: TIM4_ADDRESS + 0x04 },
    smcr: Register { addr: TIM4_ADDRESS + 0x08 },
    dier: Register { addr: TIM4_ADDRESS + 0x0C },
    sr: Register { addr: TIM4_ADDRESS + 0x10 },
    egr: Register { addr: TIM4_ADDRESS + 0x14 },
    ccmr1: Register { addr: TIM4_ADDRESS + 0x18 },
    ccmr2: Register { addr: TIM4_ADDRESS + 0x1C },
    ccer: Register { addr: TIM4_ADDRESS + 0x20 },
    cnt: Register { addr: TIM4_ADDRESS + 0x24 },
    psc: Register { addr: TIM4_ADDRESS + 0x28 },
    arr: Register { addr: TIM4_ADDRESS + 0x2C },
    rcr: EMPTY_REGISTER,
    ccr1: Register { addr: TIM4_ADDRESS + 0x34 },
    ccr2: Register { addr: TIM4_ADDRESS + 0x38 },
    ccr3: Register { addr: TIM4_ADDRESS + 0x3C },
    ccr4: Register { addr: TIM4_ADDRESS + 0x40 },
    bdtr: EMPTY_REGISTER,
    dcr: Register { addr: TIM4_ADDRESS + 0x48 },
    dmar: Register { addr: TIM4_ADDRESS + 0x4C },
    or: EMPTY_REGISTER,
};

#[allow(dead_code)]
pub const TIM5: TimTypedef = TimTypedef {
    cr1: Register { addr: TIM5_ADDRESS },
    cr2: Register { addr: TIM5_ADDRESS + 0x04 },
    smcr: Register { addr: TIM5_ADDRESS + 0x08 },
    dier: Register { addr: TIM5_ADDRESS + 0x0C },
    sr: Register { addr: TIM5_ADDRESS + 0x10 },
    egr: Register { addr: TIM5_ADDRESS + 0x14 },
    ccmr1: Register { addr: TIM5_ADDRESS + 0x18 },
    ccmr2: Register { addr: TIM5_ADDRESS + 0x1C },
    ccer: Register { addr: TIM5_ADDRESS + 0x20 },
    cnt: Register { addr: TIM5_ADDRESS + 0x24 },
    psc: Register { addr: TIM5_ADDRESS + 0x28 },
    arr: Register { addr: TIM5_ADDRESS + 0x2C },
    rcr: EMPTY_REGISTER,
    ccr1: Register { addr: TIM5_ADDRESS + 0x34 },
    ccr2: Register { addr: TIM5_ADDRESS + 0x38 },
    ccr3: Register { addr: TIM5_ADDRESS + 0x3C },
    ccr4: Register { addr: TIM5_ADDRESS + 0x40 },
    bdtr: EMPTY_REGISTER,
    dcr: Register { addr: TIM5_ADDRESS + 0x48 },
    dmar: Register { addr: TIM5_ADDRESS + 0x4C },
    or: Register { addr: TIM5_ADDRESS + 0x50 },
};

#[allow(dead_code)]
pub const TIM6: TimTypedef = TimTypedef {
    cr1: Register{addr:TIM6_ADDRESS},
    cr2: Register{addr:TIM6_ADDRESS + 0x04},
    smcr: EMPTY_REGISTER,
    dier: Register{addr:TIM6_ADDRESS + 0x0C},
    sr: Register{addr:TIM6_ADDRESS + 0x10},
    egr: Register{addr:TIM6_ADDRESS + 0x14},
    ccmr1: EMPTY_REGISTER,
    ccmr2: EMPTY_REGISTER,
    ccer: EMPTY_REGISTER,
    cnt: Register{addr:TIM6_ADDRESS + 0x24},
    psc: Register{addr:TIM6_ADDRESS + 0x28},
    arr: Register{addr:TIM6_ADDRESS + 0x2C},
    rcr: EMPTY_REGISTER,
    ccr1: EMPTY_REGISTER,
    ccr2: EMPTY_REGISTER,
    ccr3: EMPTY_REGISTER,
    ccr4: EMPTY_REGISTER,
    bdtr: EMPTY_REGISTER,
    dcr: EMPTY_REGISTER,
    dmar: EMPTY_REGISTER,
    or: EMPTY_REGISTER,
};

#[allow(dead_code)]
pub const TIM7: TimTypedef = TimTypedef {
    cr1: Register{addr:TIM7_ADDRESS},
    cr2: Register{addr:TIM7_ADDRESS + 0x04},
    smcr: EMPTY_REGISTER,
    dier: Register{addr:TIM7_ADDRESS + 0x0C},
    sr: Register{addr:TIM7_ADDRESS + 0x10},
    egr: Register{addr:TIM7_ADDRESS + 0x14},
    ccmr1: EMPTY_REGISTER,
    ccmr2: EMPTY_REGISTER,
    ccer: EMPTY_REGISTER,
    cnt: Register{addr:TIM7_ADDRESS + 0x24},
    psc: Register{addr:TIM7_ADDRESS + 0x28},
    arr: Register{addr:TIM7_ADDRESS + 0x2C},
    rcr: EMPTY_REGISTER,
    ccr1: EMPTY_REGISTER,
    ccr2: EMPTY_REGISTER,
    ccr3: EMPTY_REGISTER,
    ccr4: EMPTY_REGISTER,
    bdtr: EMPTY_REGISTER,
    dcr: EMPTY_REGISTER,
    dmar: EMPTY_REGISTER,
    or: EMPTY_REGISTER,
};

#[allow(dead_code)]
pub const TIM8: TimTypedef = TimTypedef {
    cr1: Register{addr:TIM8_ADDRESS},
    cr2: Register{addr:TIM8_ADDRESS + 0x04},
    smcr: Register { addr: TIM8_ADDRESS + 0x08 },
    dier: Register{addr:TIM8_ADDRESS + 0x0C},
    sr: Register{addr:TIM8_ADDRESS + 0x10},
    egr: Register{addr:TIM8_ADDRESS + 0x14},
    ccmr1: Register { addr: TIM8_ADDRESS + 0x18 },
    ccmr2: Register { addr: TIM8_ADDRESS + 0x1C },
    ccer: Register { addr: TIM8_ADDRESS + 0x20 },
    cnt: Register{addr:TIM8_ADDRESS + 0x24},
    psc: Register{addr:TIM8_ADDRESS + 0x28},
    arr: Register{addr:TIM8_ADDRESS + 0x2C},
    rcr: Register { addr: TIM8_ADDRESS + 0x30 },
    ccr1: Register { addr: TIM8_ADDRESS + 0x34 },
    ccr2: Register { addr: TIM8_ADDRESS + 0x38 },
    ccr3: Register { addr: TIM8_ADDRESS + 0x3C },
    ccr4: Register { addr: TIM8_ADDRESS + 0x40 },
    bdtr: Register { addr: TIM8_ADDRESS + 0x44 },
    dcr: Register { addr: TIM8_ADDRESS + 0x48 },
    dmar: Register { addr: TIM8_ADDRESS + 0x4C },
    or: EMPTY_REGISTER,
};

#[allow(dead_code)]
pub const TIM9: TimTypedef = TimTypedef {
    cr1: Register { addr: TIM9_ADDRESS },
    cr2: Register { addr: TIM9_ADDRESS + 0x04 },
    smcr: Register { addr: TIM9_ADDRESS + 0x08 },
    dier: Register { addr: TIM9_ADDRESS + 0x0C },
    sr: Register { addr: TIM9_ADDRESS + 0x10 },
    egr: Register { addr: TIM9_ADDRESS + 0x14 },
    ccmr1: Register { addr: TIM9_ADDRESS + 0x18 },
    ccmr2: EMPTY_REGISTER,
    ccer: Register { addr: TIM9_ADDRESS + 0x20 },
    cnt: Register { addr: TIM9_ADDRESS + 0x24 },
    psc: Register { addr: TIM9_ADDRESS + 0x28 },
    arr: Register { addr: TIM9_ADDRESS + 0x2C },
    rcr: EMPTY_REGISTER,
    ccr1: Register { addr: TIM9_ADDRESS + 0x34 },
    ccr2: Register { addr: TIM9_ADDRESS + 0x38 },
    ccr3: EMPTY_REGISTER,
    ccr4: EMPTY_REGISTER,
    bdtr: EMPTY_REGISTER,
    dcr: EMPTY_REGISTER,
    dmar: EMPTY_REGISTER,
    or: EMPTY_REGISTER,
};

#[allow(dead_code)]
pub const TIM10: TimTypedef = TimTypedef {
    cr1: Register { addr: TIM10_ADDRESS },
    cr2: Register { addr: TIM10_ADDRESS + 0x04 },
    smcr: Register { addr: TIM10_ADDRESS + 0x08 },
    dier: Register { addr: TIM10_ADDRESS + 0x0C },
    sr: Register { addr: TIM10_ADDRESS + 0x10 },
    egr: Register { addr: TIM10_ADDRESS + 0x14 },
    ccmr1: Register { addr: TIM10_ADDRESS + 0x18 },
    ccmr2: EMPTY_REGISTER,
    ccer: Register { addr: TIM10_ADDRESS + 0x20 },
    cnt: Register { addr: TIM10_ADDRESS + 0x24 },
    psc: Register { addr: TIM10_ADDRESS + 0x28 },
    arr: Register { addr: TIM10_ADDRESS + 0x2C },
    rcr: EMPTY_REGISTER,
    ccr1: Register { addr: TIM10_ADDRESS + 0x34 },
    ccr2: EMPTY_REGISTER,
    ccr3: EMPTY_REGISTER,
    ccr4: EMPTY_REGISTER,
    bdtr: EMPTY_REGISTER,
    dcr: EMPTY_REGISTER,
    dmar: EMPTY_REGISTER,
    or: Register { addr: TIM10_ADDRESS + 0x50 },
};

#[allow(dead_code)]
pub const TIM11: TimTypedef = TimTypedef {
    cr1: Register { addr: TIM11_ADDRESS },
    cr2: Register { addr: TIM11_ADDRESS + 0x04 },
    smcr: Register { addr: TIM11_ADDRESS + 0x08 },
    dier: Register { addr: TIM11_ADDRESS + 0x0C },
    sr: Register { addr: TIM11_ADDRESS + 0x10 },
    egr: Register { addr: TIM11_ADDRESS + 0x14 },
    ccmr1: Register { addr: TIM11_ADDRESS + 0x18 },
    ccmr2: EMPTY_REGISTER,
    ccer: Register { addr: TIM11_ADDRESS + 0x20 },
    cnt: Register { addr: TIM11_ADDRESS + 0x24 },
    psc: Register { addr: TIM11_ADDRESS + 0x28 },
    arr: Register { addr: TIM11_ADDRESS + 0x2C },
    rcr: EMPTY_REGISTER,
    ccr1: Register { addr: TIM11_ADDRESS + 0x34 },
    ccr2: EMPTY_REGISTER,
    ccr3: EMPTY_REGISTER,
    ccr4: EMPTY_REGISTER,
    bdtr: EMPTY_REGISTER,
    dcr: EMPTY_REGISTER,
    dmar: EMPTY_REGISTER,
    or: Register { addr: TIM11_ADDRESS + 0x50 },
};

#[allow(dead_code)]
pub const TIM12: TimTypedef = TimTypedef {
    cr1: Register { addr: TIM12_ADDRESS },
    cr2: Register { addr: TIM12_ADDRESS + 0x04 },
    smcr: Register { addr: TIM12_ADDRESS + 0x08 },
    dier: Register { addr: TIM12_ADDRESS + 0x0C },
    sr: Register { addr: TIM12_ADDRESS + 0x10 },
    egr: Register { addr: TIM12_ADDRESS + 0x14 },
    ccmr1: Register { addr: TIM12_ADDRESS + 0x18 },
    ccmr2: EMPTY_REGISTER,
    ccer: Register { addr: TIM12_ADDRESS + 0x20 },
    cnt: Register { addr: TIM12_ADDRESS + 0x24 },
    psc: Register { addr: TIM12_ADDRESS + 0x28 },
    arr: Register { addr: TIM12_ADDRESS + 0x2C },
    rcr: EMPTY_REGISTER,
    ccr1: Register { addr: TIM12_ADDRESS + 0x34 },
    ccr2: Register { addr: TIM12_ADDRESS + 0x38 },
    ccr3: EMPTY_REGISTER,
    ccr4: EMPTY_REGISTER,
    bdtr: EMPTY_REGISTER,
    dcr: EMPTY_REGISTER,
    dmar: EMPTY_REGISTER,
    or: EMPTY_REGISTER,
};

#[allow(dead_code)]
pub const TIM13: TimTypedef = TimTypedef {
    cr1: Register { addr: TIM13_ADDRESS },
    cr2: Register { addr: TIM13_ADDRESS + 0x04 },
    smcr: Register { addr: TIM13_ADDRESS + 0x08 },
    dier: Register { addr: TIM13_ADDRESS + 0x0C },
    sr: Register { addr: TIM13_ADDRESS + 0x10 },
    egr: Register { addr: TIM13_ADDRESS + 0x14 },
    ccmr1: Register { addr: TIM13_ADDRESS + 0x18 },
    ccmr2: EMPTY_REGISTER,
    ccer: Register { addr: TIM13_ADDRESS + 0x20 },
    cnt: Register { addr: TIM13_ADDRESS + 0x24 },
    psc: Register { addr: TIM13_ADDRESS + 0x28 },
    arr: Register { addr: TIM13_ADDRESS + 0x2C },
    rcr: EMPTY_REGISTER,
    ccr1: Register { addr: TIM13_ADDRESS + 0x34 },
    ccr2: EMPTY_REGISTER,
    ccr3: EMPTY_REGISTER,
    ccr4: EMPTY_REGISTER,
    bdtr: EMPTY_REGISTER,
    dcr: EMPTY_REGISTER,
    dmar: EMPTY_REGISTER,
    or: Register { addr: TIM13_ADDRESS + 0x50 },
};

#[allow(dead_code)]
pub const TIM14: TimTypedef = TimTypedef {
    cr1: Register { addr: TIM14_ADDRESS },
    cr2: Register { addr: TIM14_ADDRESS + 0x04 },
    smcr: Register { addr: TIM14_ADDRESS + 0x08 },
    dier: Register { addr: TIM14_ADDRESS + 0x0C },
    sr: Register { addr: TIM14_ADDRESS + 0x10 },
    egr: Register { addr: TIM14_ADDRESS + 0x14 },
    ccmr1: Register { addr: TIM14_ADDRESS + 0x18 },
    ccmr2: EMPTY_REGISTER,
    ccer: Register { addr: TIM14_ADDRESS + 0x20 },
    cnt: Register { addr: TIM14_ADDRESS + 0x24 },
    psc: Register { addr: TIM14_ADDRESS + 0x28 },
    arr: Register { addr: TIM14_ADDRESS + 0x2C },
    rcr: EMPTY_REGISTER,
    ccr1: Register { addr: TIM14_ADDRESS + 0x34 },
    ccr2: EMPTY_REGISTER,
    ccr3: EMPTY_REGISTER,
    ccr4: EMPTY_REGISTER,
    bdtr: EMPTY_REGISTER,
    dcr: EMPTY_REGISTER,
    dmar: EMPTY_REGISTER,
    or: Register { addr: TIM14_ADDRESS + 0x50 },
};

#[allow(dead_code)]
pub struct TimTypedef {
    // A struct that represents stm32f4xx timer registers.
    pub cr1: Register,  //Address offset: 0x00
    pub cr2: Register,  //Address offset: 0x04
    pub smcr: Register, //Address offset: 0x08
    pub dier: Register, //Address offset: 0x0C
    pub sr: Register,   //Address offset: 0x10
    pub egr: Register,  //Address offset: 0x14
    pub ccmr1: Register,//Address offset: 0x18
    pub ccmr2: Register,//Address offset: 0x1C
    pub ccer: Register, //Address offset: 0x20
    pub cnt: Register,  //Address offset: 0x24
    pub psc: Register,  //Address offset: 0x28
    pub arr: Register,  //Address offset: 0x2C
    pub rcr: Register,  //Address offset: 0x30
    pub ccr1: Register, //Address offset: 0x34
    pub ccr2: Register, //Address offset: 0x38
    pub ccr3: Register, //Address offset: 0x3C
    pub ccr4: Register, //Address offset: 0x40
    pub bdtr: Register, //Address offset: 0x44
    pub dcr: Register,  //Address offset: 0x48
    pub dmar: Register, //Address offset: 0x4C
    pub or: Register,   //Address offset: 0x50
}



