use core::ops::Range;
use crate::memory;

pub trait RunTimeInit {
    unsafe fn runtime_init(&self) -> ! {
        zero_bss();

        crate::kernel_init()
    }
}

struct Traitor;

impl RunTimeInit for Traitor{
    unsafe fn runtime_init(&self) -> ! {
        zero_bss();

        crate::kernel_init()
    }
    
}

unsafe fn bss_range() -> Range<*mut usize>{
    extern "C" {
        static mut __bss_start: usize;
        static mut __bss_end: usize;
    }

    Range{
        start:&mut __bss_start,
        end:&mut __bss_end
    }
}

#[inline(always)]
unsafe fn zero_bss(){
    memory::zero_volatile(bss_range());
}

pub fn get() -> &'static dyn RunTimeInit {
    &Traitor {}
}