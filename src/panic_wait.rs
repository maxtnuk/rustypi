// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>

//! A panic handler that infinitely waits.

use crate::cpu;
use core::panic::PanicInfo;
use crate::println;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    if let Some(args) = _info.message() {
        println!("\nKernel panic: {}", args);
    } else {
        println!("\nKernel panic!");
    }
    cpu::wait_forever()
}