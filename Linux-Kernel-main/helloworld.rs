use core::fmt::Write;

#[no_mangle]
pub extern "C" fn init() {
    // Print "Hello, world!" to the kernel log
    let _ = writeln!(unsafe { crate::console::CONSOLE }, "Hello, world!");
}
