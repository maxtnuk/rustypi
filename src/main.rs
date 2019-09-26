#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use rt;

#[no_mangle]
pub fn main() -> ! {
    let _x = 42;

    loop {}
}
