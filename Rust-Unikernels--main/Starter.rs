#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Define the entry point function for Xen
#[no_mangle]
pub extern "C" fn xen_start_info() -> ! {
    // Your code goes here!
    loop {}
}
