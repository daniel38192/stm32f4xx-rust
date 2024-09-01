use alloc::format;
use alloc::string::String;
use crate::stm32f4xx::peripherals::rtcdef::RTC;
use crate::stm32f4xx::peripherals::pwrdef::PWR;
use crate::stm32f4xx::peripherals::rccdef::RCC;

// 21st century.
const MILLENNIUM: i32 = 2000;

pub struct Date {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32,
    weekday: i32
}

impl Date {
    #[allow(dead_code)]
    pub fn now() -> Self {
        let (year, month, day, weekday) = read_date_in_registers();
        let (hour, minute, second) = read_time_in_registers();
        Date {
            year,
            month,
            day,
            hour,
            minute,
            second,
            weekday
        }
    }
    #[allow(dead_code)]
    pub fn full_date(&self) -> String {
        //Example Monday, 2006-01-02 15:04:05
        format!("{}, {} {}", self.weekday(), self.date_only(), self.time_only())
    }
    pub fn date_only(&self) -> String {
        let f_month;

        if self.month < 10 {
           f_month = format!("0{}", self.month)
        } else {
            f_month = format!("{}", self.month)
        }

        let f_day;
        if self.day < 10 {
            f_day = format!("0{}", self.day)
        } else {
            f_day = format!("{}", self.day)
        }
        //Example: 2006-01-02
        format!("{}-{}-{}", self.year, f_month, f_day)
    }
    pub fn time_only(&self) -> String {
        let f_hour;

        if self.hour < 10 {
            f_hour = format!("0{}", self.hour)
        } else {
            f_hour = format!("{}", self.hour)
        }

        let f_minute;
        if self.minute < 10 {
            f_minute = format!("0{}", self.minute)
        } else {
            f_minute = format!("{}", self.minute)
        }

        let f_second;
        if self.second < 10 {
            f_second = format!("0{}", self.second)
        } else {
            f_second = format!("{}", self.second)
        }
        // Example: 15:04:05
        format!("{}:{}:{}", f_hour, f_minute, f_second)
    }
    #[allow(dead_code)]
    pub fn time_type_kitchen(&self) -> String {
        let am_or_pm;
        let f_hour;

        if self.hour > 12 {
            f_hour = format!("{}", self.hour - 12);
            am_or_pm = String::from("PM")
        } else if self.hour == 12 {
            f_hour = format!("{}", self.hour);
            am_or_pm = String::from("PM")
        } else {
            f_hour = format!("{}", self.hour);
            am_or_pm = String::from("AM")
        }

        let f_minute;
        if self.minute < 10 {
            f_minute = format!("0{}", self.minute)
        } else {
            f_minute = format!("{}", self.minute)
        }

        //Example 3:04PM
        format!("{}:{}{}", f_hour, f_minute, am_or_pm)
    }
    fn weekday(&self) -> &str {
        match self.weekday {
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            6 => "Saturday",
            7 => "Sunday",
            _ => "",
        }
    }
}

pub fn read_date_in_registers() -> (i32, i32, i32, i32) {

    let date_result = RTC.dr.get();
    // copy year tens and year units
    let actual_year = ((((date_result & (0xF << 20)) >> 20) * 10) + ((date_result & (0xF << 16)) >> 16)) as i32 + MILLENNIUM;
    // copy weekday unit
    let actual_weekday = ((date_result & (0x7 << 13)) >> 13) as i32;
    // copy month tens and month units
    let actual_month = ((((date_result & (1 << 12)) >> 12) * 10) + ((date_result & (0xF << 8)) >> 8)) as i32;
    // copy day tens and day units
    let actual_day = ((((date_result & (0x3 << 4)) >> 4) * 10) + ((date_result & (0xF << 0)) >> 0)) as i32;
    (actual_year, actual_month, actual_day, actual_weekday)
}

pub fn read_time_in_registers() -> (i32, i32, i32){
    let time_result = RTC.tr.get();
    //copy hour tens and hour units
    let actual_hour = ((((time_result & (0x3 << 20)) >> 20) * 10) + ((time_result & (0xF << 16)) >> 16)) as i32;
    //copy minute tens and minute units
    let actual_minute = ((((time_result & (0x7 << 12)) >> 12) * 10) + ((time_result & (0xF << 8)) >> 8)) as i32;
    // copy second tens and second units
    let actual_second = ((((time_result & (0x7 << 4)) >> 4) * 10) + ((time_result & (0xF << 0)) >> 0)) as i32;
    (actual_hour, actual_minute, actual_second)
}

pub fn rtc_configure_once(){
    // After a system reset, the application can read the INITS flag in the RTC_ISR register to
    // check if the calendar has been initialized or not. If this flag equals 0, the calendar has not
    // been initialized since the year field is set at its backup domain reset default value (0x00).
    // To read the calendar after initialization, the software must first check that the RSF flag is set
    // in the RTC_ISR register.

    if !RTC.isr.has_bits(1 << 4) {
        disable_backup_write_domain_protection();
        enable_low_speed_external_clock();
        configure_clock_for_rtc();
        disable_rtc_write_protection();
        calendar_enter_init_mode();
        set_prescaler();
        calendar_load_date_init_values(2, 4, 0, 9, 0, 1, 7);
        calendar_load_hour_init_values(1,0, 0,2,0,0);
        exit_init_mode();
    }
}

fn disable_backup_write_domain_protection(){
    // Enable PWR interface
    RCC.apb1enr.set_bits(1 << 28);

    // Disable backup domain protection in PWR_CR register bit 8
    PWR.cr.set_bits(1 << 8);

    // a dummy read operation to the PWR_CR register is required just after writing to the DBP bit.
    // this is required.
    #[allow(unused_variables)]
    let temp = PWR.cr.get();

}

fn enable_low_speed_external_clock(){
    // Enable LSE in bit 0 RCC_BDCR
    RCC.bdcr.set_bits(1 << 0);

    // Wait for LSERDY flag bit 1
    while !RCC.bdcr.has_bits(1 << 1) { }
}

fn configure_clock_for_rtc(){
    // Select LSE clock in RCC_BDCR
    RCC.bdcr.set_bits(1 << 8);
    // Enable RTC clock in RCC_BDCR
    RCC.bdcr.set_bits(1 << 15);
}

fn disable_rtc_write_protection(){
    // The following steps are required to unlock the write protection on all the RTC registers
    // except for RTC_ISR[13:8], RTC_TAFCR, and RTC_BKPxR.

    // 1.Write ‘0xCA’ into the RTC_WPR register.
    RTC.wpr.set(0xCA);
    // 2.Write ‘0x53’ into the RTC_WPR register.
    RTC.wpr.set(0x53);

}

fn calendar_enter_init_mode(){
    // Set INIT bit to 1 in the RTC_ISR register to enter initialization mode. In this mode, the
    // calendar counter is stopped and its value can be updated.
    RTC.isr.set(1 << 7);

    // Poll INITF bit of in the RTC_ISR register. The initialization phase mode is entered when
    // INITF is set to 1. It takes from 1 to 2 RTCCLK clock cycles (due to clock
    // synchronization).
    while !RTC.isr.has_bits(1 << 6) { }

}

fn set_prescaler(){
    // To generate a 1 Hz clock for the calendar counter, program first the synchronous
    // prescaler factor in RTC_PRER register, and then program the asynchronous prescaler
    // factor. Even if only one of the two fields needs to be changed, 2 separate write
    // accesses must be performed to the RTC_PRER register.
    RTC.prer.set_bits(255 << 0);
    RTC.prer.set_bits(127 << 16);
}


fn calendar_load_hour_init_values(hour_tens: u32, hour_units: u32, minute_tens: u32, minute_units: u32, second_tens: u32, second_units: u32) {
    // Load the initial time and date values in the shadow registers (RTC_TR and RTC_DR),
    // and configure the time format (12 or 24 hours) through the FMT bit in the RTC_CR
    // register.
    RTC.tr.set((hour_tens << 20)|(hour_units << 16)|(minute_tens << 12)|(minute_units << 8)|(second_tens << 4)|(second_units << 0))
}

fn calendar_load_date_init_values(year_tens: u32, year_units: u32, month_tens: u32, month_units: u32, day_tens: u32, day_units: u32, weekday_units: u32){
    // Load the initial time and date values in the shadow registers (RTC_TR and RTC_DR),
    // and configure the time format (12 or 24 hours) through the FMT bit in the RTC_CR
    // register.
    RTC.dr.set((year_tens << 20)|(year_units << 16)|(weekday_units << 13)|(month_tens << 12)|(month_units << 8)|(day_tens << 4)|(day_units << 0))
}

fn exit_init_mode(){
    // Exit the initialization mode by clearing the INIT bit. The actual calendar counter value is
    // then automatically loaded and the counting restarts after 4 RTCCLK clock cycles.
    RTC.isr.clear_bits(1 << 7);
}