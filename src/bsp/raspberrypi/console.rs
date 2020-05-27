use core::fmt;
use core::fmt::{Arguments, Write};

use crate::{console, synchronization::NullLock};
use crate::synchronization::interface::Mutex;

const UART_ADDRESS: *mut u8 = 0x3F20_1000 as *mut u8;

pub struct QEMUOutputInner {
    chars_written: usize,
}

pub struct QEMUOutput {
    inner: NullLock<QEMUOutputInner>
}

impl QEMUOutputInner {
    pub const fn new() -> Self {
        Self {
            chars_written: 0
        }
    }
    fn write_char(&mut self, c: char) {
        unsafe {
            core::ptr::write_volatile(UART_ADDRESS, c as u8);
        }
        self.chars_written += 1;
    }
}

impl QEMUOutput {
    pub const fn new() -> Self {
        Self {
            inner: NullLock::new(QEMUOutputInner::new())
        }
    }
}

impl fmt::Write for QEMUOutputInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            if c == '\n' {
                self.write_char('\r')
            }
            self.write_char(c);
        }

        Ok(())
    }
}


impl console::interface::Write for QEMUOutput {
    fn write_fmt(&self, args: Arguments) -> fmt::Result {
        let mut r = &self.inner;
        r.lock(|inner| inner.write_fmt(args))
    }
}

impl console::interface::Statistics for QEMUOutput {
    fn chars_written(&self) -> usize {
        let mut r = &self.inner;
        r.lock(|inner| inner.chars_written)
    }
}


static QEMU_OUTPUT: QEMUOutput = QEMUOutput::new();

pub fn console() -> &'static impl console::interface::All {
    &QEMU_OUTPUT
}