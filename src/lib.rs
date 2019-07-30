#![no_std]

pub mod delay;
pub mod errors;
pub mod gpio;
pub mod i2c;
pub mod serial;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        esp32_sys::abort();
        core::hint::unreachable_unchecked();
    }
}