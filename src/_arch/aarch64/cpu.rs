// Assembly counterpart to this file.
global_asm!(include_str!("cpu.S"));

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

#[inline(always)]
pub fn wait_forever() -> ! {
    unsafe {
        loop {
            llvm_asm!("wfe"
                    :             // outputs
                    :             // inputs
                    :             // clobbers
                    : "volatile") // options
        }
    }
}