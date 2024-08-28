
use crate::stm32f4xx::core::delay::non_exact_time_delay;
use crate::stm32f4xx::drivers::gpio::*;

pub struct Lcd8bit{
    pub register_select: Gpio,
    pub read_write: Gpio,
    pub enable: Gpio,
    pub d0: Gpio,
    pub d1: Gpio,
    pub d2: Gpio,
    pub d3: Gpio,
    pub d4: Gpio,
    pub d5: Gpio,
    pub d6: Gpio,
    pub d7: Gpio,
}

const TYPE_GPIO_LCD_PORT: GpioConfig = GpioConfig {
    mode: OUTPUT,
    otyper: PUSH_PULL,
    ospeedr: HIGH_SPEED,
    pupdr: NO_PULL_UP_DOWN,
    alt_func_sel: 0
};

impl Lcd8bit {
    #[allow(dead_code)]
    pub fn configure(&self){
        self.init_pins();
        non_exact_time_delay(400000);
        self.send_command(0x38);
        self.send_command(0xE);
        self.send_command(0x6);
    }
    #[allow(dead_code)]
    pub fn print(&self, str: &str){
        let bytes = str.as_bytes();

        for c in bytes {
            let b = *c as char;
            self.send_character(b);
        }
    }
    #[allow(dead_code)]
    pub fn set_cursor(&self, x: i32, y: i32){
        self.send_command(0b10000000u8 + (x as u8 -1) + ((y as u8-1)* 64));
    }
    pub fn send_character(&self, c: char){
        // RS -> 0
        // RW -> 1
        self.read_write.low();
        self.register_select.high();

        // first pause
        non_exact_time_delay(400);

        // EN -> 1
        // c -> D0 - D7
        self.enable.high();
        self.toggle_pins_char(c);

        // second pause
        non_exact_time_delay(800);

        // EN -> 0
        self.enable.low();

        // third pause
        non_exact_time_delay(200);


    }
    pub fn send_command(&self, cmd: u8){
        // RS -> 0
        // RW -> 0
        self.read_write.low();
        self.register_select.low();

        // first pause
        non_exact_time_delay(400);

        // EN -> 1
        // c -> D0 - D7
        self.enable.high();
        self.toggle_pins(cmd);

        // second pause
        non_exact_time_delay(800);

        // EN -> 0
        self.enable.low();

        // third pause
        non_exact_time_delay(200);

    }
    fn toggle_pins_char(&self, data: char){
        self.toggle_pins(data as u8)
    }
    fn toggle_pins(&self, data: u8) {
        if (data & 0b00000001) > 0 {
            self.d0.high()
        } else {
            self.d0.low()
        }
        if (data & 0b00000010) > 0 {
            self.d1.high()
        } else {
            self.d1.low()
        }
        if (data & 0b00000100) > 0 {
            self.d2.high()
        } else {
            self.d2.low()
        }
        if (data & 0b00001000) > 0 {
            self.d3.high()
        } else {
            self.d3.low()
        }
        if (data & 0b00010000) > 0 {
            self.d4.high()
        } else {
            self.d4.low()
        }
        if (data & 0b00100000) > 0 {
            self.d5.high()
        } else {
            self.d5.low()
        }
        if (data & 0b01000000) > 0 {
            self.d6.high()
        } else {
            self.d6.low()
        }
        if (data & 0b10000000) > 0 {
            self.d7.high()
        } else {
            self.d7.low()
        }
    }
    fn init_pins(&self){
        self.register_select.configure(TYPE_GPIO_LCD_PORT);
        self.read_write.configure(TYPE_GPIO_LCD_PORT);
        self.enable.configure(TYPE_GPIO_LCD_PORT);
        self.d0.configure(TYPE_GPIO_LCD_PORT);
        self.d1.configure(TYPE_GPIO_LCD_PORT);
        self.d2.configure(TYPE_GPIO_LCD_PORT);
        self.d3.configure(TYPE_GPIO_LCD_PORT);
        self.d4.configure(TYPE_GPIO_LCD_PORT);
        self.d5.configure(TYPE_GPIO_LCD_PORT);
        self.d6.configure(TYPE_GPIO_LCD_PORT);
        self.d7.configure(TYPE_GPIO_LCD_PORT);
    }
}
