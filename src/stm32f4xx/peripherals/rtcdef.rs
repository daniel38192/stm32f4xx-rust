use crate::stm32f4xx::core::register::Register;

const RTC_ADDRESS: u32 = 0x40002800;

#[allow(dead_code)]
pub const RTC: RtcTypedef = RtcTypedef {
    tr: Register {addr:RTC_ADDRESS},
    dr: Register {addr:RTC_ADDRESS + 0x04},
    cr: Register {addr:RTC_ADDRESS + 0x08},
    isr: Register {addr:RTC_ADDRESS + 0x0C},
    prer: Register {addr:RTC_ADDRESS + 0x10},
    wutr: Register {addr:RTC_ADDRESS + 0x14},
    calibr: Register {addr:RTC_ADDRESS + 0x18},
    alrmar: Register {addr:RTC_ADDRESS + 0x1C},
    alrmbr: Register {addr:RTC_ADDRESS + 0x20},
    wpr: Register {addr:RTC_ADDRESS + 0x24},
    ssr: Register {addr:RTC_ADDRESS + 0x28},
    shiftr: Register {addr:RTC_ADDRESS + 0x2C},
    tstr: Register {addr:RTC_ADDRESS + 0x30},
    tsdr: Register {addr:RTC_ADDRESS + 0x34},
    tsssr: Register {addr:RTC_ADDRESS + 0x38},
    calr: Register {addr:RTC_ADDRESS + 0x3C},
    tafcr: Register {addr:RTC_ADDRESS + 0x40},
    alrmassr: Register {addr:RTC_ADDRESS + 0x44},
    alrmbssr: Register {addr:RTC_ADDRESS + 0x48},
    bkpxr: [
        Register {addr:RTC_ADDRESS + 0x50},
        Register {addr:RTC_ADDRESS + 0x54},
        Register {addr:RTC_ADDRESS + 0x58},
        Register {addr:RTC_ADDRESS + 0x5C},
        Register {addr:RTC_ADDRESS + 0x60},
        Register {addr:RTC_ADDRESS + 0x64},
        Register {addr:RTC_ADDRESS + 0x68},
        Register {addr:RTC_ADDRESS + 0x6C},
        Register {addr:RTC_ADDRESS + 0x70},
        Register {addr:RTC_ADDRESS + 0x74},
        Register {addr:RTC_ADDRESS + 0x78},
        Register {addr:RTC_ADDRESS + 0x7C},
        Register {addr:RTC_ADDRESS + 0x80},
        Register {addr:RTC_ADDRESS + 0x84},
        Register {addr:RTC_ADDRESS + 0x88},
        Register {addr:RTC_ADDRESS + 0x8C},
        Register {addr:RTC_ADDRESS + 0x90},
        Register {addr:RTC_ADDRESS + 0x94},
        Register {addr:RTC_ADDRESS + 0x98},
        Register {addr:RTC_ADDRESS + 0x9C}
],
};
#[allow(dead_code)]
pub struct RtcTypedef {
    pub tr: Register,       //Address offset: 0x00
    pub dr: Register,       //Address offset: 0x04
    pub cr: Register,       //Address offset: 0x08
    pub isr: Register,      //Address offset: 0x0C
    pub prer: Register,     //Address offset: 0x10
    pub wutr: Register,     //Address offset: 0x14
    pub calibr: Register,   //Address offset: 0x18
    pub alrmar: Register,   //Address offset: 0x1C
    pub alrmbr: Register,   //Address offset: 0x20
    pub wpr: Register,      //Address offset: 0x24
    pub ssr: Register,      //Address offset: 0x28
    pub shiftr: Register,   //Address offset: 0x2C
    pub tstr: Register,     //Address offset: 0x30
    pub tsdr: Register,     //Address offset: 0x34
    pub tsssr: Register,    //Address offset: 0x38
    pub calr: Register,     //Address offset: 0x3C
    pub tafcr: Register,    //Address offset: 0x40
    pub alrmassr: Register, //Address offset: 0x44
    pub alrmbssr: Register, //Address offset: 0x48
    pub bkpxr: [Register; 20],  //Address offset: 0x50 to 0x9C
}