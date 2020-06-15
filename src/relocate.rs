use crate::{bsp, runtime_init};

pub unsafe fn relocate_fn<T>() -> ! {
    extern "C" {
        static __binary_start: usize;
        static __binary_end: usize;
    }

    let binary_start_addr = &__binary_start as *const usize as usize;
    let binary_end_addr = &__binary_end as *const usize as usize;
    let binary_size_in_byte = binary_end_addr - binary_start_addr;

    let mut reloc_dst_addr = binary_start_addr as *mut T;

    let mut src_addr = bsp::cpu::BOARD_DEFAULT_LOAD_ADDRESS as *const T;

    let n = binary_size_in_byte / core::mem::size_of::<T>();
    for _ in 0..n {
        use core::ptr;

        ptr::write_volatile::<T>(reloc_dst_addr, ptr::read_volatile::<T>(src_addr));
        reloc_dst_addr = reloc_dst_addr.offset(1);
        src_addr = src_addr.offset(1);
    }

    runtime_init::get().runtime_init()
}
