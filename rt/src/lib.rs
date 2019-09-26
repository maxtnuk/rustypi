#![no_std] // don't link the Rust standard library
#![feature(global_asm)]

use core::panic::PanicInfo;
use core::{mem, ptr};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            // type check the given path
            let f: fn() -> ! = $path;

            f()
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "C" {
        // Boundaries of the .bss section, provided by the linker script
        static mut __bss_start: u64;
        static mut __bss_end: u64;
    }
    let mut sbss: *mut u64 = &mut __bss_start;
    let ebss: *mut u64 = &mut __bss_end;

    while sbss < ebss {
        ptr::write_volatile(sbss, mem::zeroed());
        sbss = sbss.offset(1);
    }

    extern "Rust" {
        fn main() -> !;
    }

    main()
}

#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

global_asm!(include_str!("boot.S"));
