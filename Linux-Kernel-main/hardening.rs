use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // Disable loading of kernel modules
    let mut f = File::create("/etc/modprobe.d/hardening.conf")?;
    f.write_all(b"install cramfs /bin/true\n")?;
    f.write_all(b"install freevxfs /bin/true\n")?;

    // Enable kernel address space layout randomization (ASLR)
    let mut f = File::create("/proc/sys/kernel/randomize_va_space")?;
    f.write_all(b"2")?;

    // Disable kernel core dumps
    let mut f = File::create("/etc/security/limits.conf")?;
    f.write_all(b"* hard core 0")?;

    Ok(())
}
