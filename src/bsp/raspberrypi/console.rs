use crate::console;
use core::fmt;

struct QEMUOutput;

const SAMPLE:*mut u8  = 0x3F20_1000 as *mut u8;

impl fmt::Write for QEMUOutput{
    fn write_str(&mut self, s: &str)-> fmt::Result{
        for c in s.chars(){
            unsafe{
                core::ptr::write_volatile(SAMPLE,c as u8)
            }
        }
        Ok(())
    }
}

pub fn console() -> impl console::interface::Write{
    QEMUOutput{}
}