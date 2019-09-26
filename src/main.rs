#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use rt::entry;

#[no_mangle]
pub fn kernel_entry() -> ! {
    let _x = 42;

    loop {}
}

entry!(kernel_entry);
