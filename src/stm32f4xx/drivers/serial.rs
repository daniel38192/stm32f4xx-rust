use libm::roundf;
use crate::stm32f4xx::drivers::gpio::*;
use crate::stm32f4xx::peripherals::gpiodef::GpioTypedef;
use crate::stm32f4xx::peripherals::rccdef::RCC;
use crate::stm32f4xx::peripherals::usartdef::*;

const FREQ_CLK: u32 = 16000000;
#[allow(dead_code)]
pub struct Serial {
    pub serial_port: UsartTypedef,
    pub gpio_port: GpioTypedef,
    pub tx_pin: u32,
    pub rx_pin: u32,
    pub baudrate: u32,
    pub alt_func_selector: u32,
}
#[allow(dead_code)]
impl Serial {
    #[allow(dead_code)]
    pub fn configure(&self){
        self.configure_pins();
        self.enable_rcc_clock();
        self.set_baudrate();
        self.configure_registers();
        self.send_byte(0);
    }

    #[allow(dead_code)]
    pub fn println(&self, st: &str){
        self.print(st);
        self.new_line()
    }

    #[allow(dead_code)]
    pub fn print(&self, st: &str){
        let bytes= st.as_bytes();

        for c in bytes {
            self.send_byte(*c);
        }
    }

    #[allow(dead_code)]
    pub fn new_line(&self){
        self.send_byte(0xA); //New line
        self.send_byte(0xD); //Carriage return
    }

    #[allow(dead_code)]
    pub fn send_char(&self, c: char){
        self.send_byte(c as u8)
    }

    #[allow(dead_code)]
    pub fn read_byte(&self) -> u8 {
        // wait for bit 5 RXNE
        if self.serial_port.sr.has_bits(1 << 5) {
            self.serial_port.dr.get() as u8
        } else {
            0
        }
    }

    pub fn send_byte(&self, data: u8){
        // load the character into the data register
        self.serial_port.dr.set(data as u32);

        // wait for transmission complete flag
        while !self.serial_port.sr.has_bits( 1 << 6) { }
    }

    fn configure_pins(&self){
        let rx_pin = Gpio {port: self.gpio_port.clone(), pin_number: 10};
        let tx_pin = Gpio {port: self.gpio_port.clone(), pin_number: 9};

        rx_pin.configure(GpioConfig {
            mode: ALTERNATE,
            otyper: PUSH_PULL,
            ospeedr: VERY_HIGH_SPEED,
            pupdr: NO_PULL_UP_DOWN,
            alt_func_sel: self.alt_func_selector,
        });
        tx_pin.configure(GpioConfig {
            mode: ALTERNATE,
            otyper: PUSH_PULL,
            ospeedr: VERY_HIGH_SPEED,
            pupdr: NO_PULL_UP_DOWN,
            alt_func_sel: self.alt_func_selector,
        });
    }

    fn set_baudrate(&self){
        let usartdiv: f32 = FREQ_CLK as f32 / (8.0 * (2.0 - 0.0) * self.baudrate as f32);
        let matissa = usartdiv as u32;
        let fraction = roundf((usartdiv - (usartdiv as i32) as f32) * 16.0) as u32;

        // first test, baudrate is fixed to 115200
        //Program USART_BRR, joining DIV_Fraction and DIV_Mantissa[11:0]
        self.serial_port.brr.set((matissa<<4)|(fraction<<0));
    }

    fn configure_registers(&self){
        //default prescaler value
        self.serial_port.gtpr.set_bits(1);

        //enable transmitter
        self.serial_port.cr1.set_bits(1 << 3);

        //enable receiver
        self.serial_port.cr1.set_bits(1 << 2);

        //enable USART
        self.serial_port.cr1.set_bits(1 << 13);
    }

    fn enable_rcc_clock(&self){
        match self.serial_port {
            USART1 => RCC.apb2enr.set_bits(1 << 4),
            USART2 => RCC.apb1enr.set_bits(1 << 17),
            USART3 => RCC.apb1enr.set_bits(1 << 18),
            UART4 => RCC.apb1enr.set_bits(1 << 19),
            UART5 => RCC.apb1enr.set_bits(1 << 20),
            USART6 => RCC.apb2enr.set_bits(1 << 5),
            _ => {}
        }
    }

}

