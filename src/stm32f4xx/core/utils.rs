use core::fmt::Write;
use core::fmt::Result;

pub struct Buffer {
    pub buff: [u8; 32],
    pub pos: usize
}

impl Buffer {
    pub fn new() -> Self {
        Buffer {
            buff: [0; 32],
            pos: 0
        }
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buff[..self.pos]).unwrap()
    }
}

impl Write for Buffer {
    fn write_str(&mut self, s: &str) -> Result {
        let bytes = s.as_bytes();
        self.buff[self.pos..self.pos + bytes.len()].copy_from_slice(bytes);
        self.pos += bytes.len();
        Ok(())
    }
}
#[allow(dead_code)]
pub fn i32_to_str(i: i32) -> Buffer {
    let mut buffer = Buffer::new();
    write!(&mut buffer, "{}", i).unwrap();
    buffer
}
#[allow(dead_code)]
pub fn i32_to_hex_str(i: i32) -> Buffer {
    let mut buffer = Buffer::new();
    write!(&mut buffer, "{:X}", i).unwrap();
    buffer
}