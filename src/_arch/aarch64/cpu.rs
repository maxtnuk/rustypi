use crate::{bsp, cpu};
pub use asm::{nop, wfe};
use cortex_a::{asm, regs::*};

#[naked]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    use crate::relocate;

    if bsp::cpu::BOOT_CORE_ID == cpu::smp::core_id() {
        SP.set(bsp::cpu::BOOT_CORE_STACK_START);
        relocate::relocate_fn::<u64>()
    } else {
        wait_forever()
    }
}

#[inline(always)]
pub fn spin_for_cycles(n: usize) {
    for _ in 0..n {
        nop();
    }
}

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        wfe();
    }
}
