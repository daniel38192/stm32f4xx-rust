
pub const EMPTY_REGISTER: Register = Register{addr: 0 };

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub struct Register {
    pub addr: u32
}

impl Register {
    #[allow(dead_code)]
    pub fn get (&self) -> u32{
        unsafe {
            *(self.addr as *mut u32)
        }
    }
    #[allow(dead_code)]
    pub fn set(&self, value: u32) {
        unsafe {
            *(self.addr as *mut u32) = value;
        }
    }
    #[allow(dead_code)]
    pub fn set_bits(&self, value: u32) {
        unsafe {
            *(self.addr as *mut u32) |= value;
        }
    }
    #[allow(dead_code)]
    pub fn clear_bits(&self, value: u32) {
        unsafe {
            *(self.addr as *mut u32) &= !value;
        }
    }
    #[allow(dead_code)]
    pub fn has_bits(&self, value: u32) -> bool {
        unsafe {
            if (*(self.addr as *mut u32) & value) > 0 {
                true
            } else {
                false
            }
        }
    }
}