//hacked together from a book and not tested at all before release do not evaluate in production I, Maxwell Seefeld take no legal responsibility for this module
//despite it being written by me I hacked stuff together from a book and then used ChatGPT solely for code review and I'm still new to the kernel developement process

use crate::nvidia_gpu::set_gpu_voltage;
use crate::intel_cpu::set_cpu_voltage;

use linux_kernel_module::{
    KernelResult, c_types::c_uint, c_types::c_void, errors::Error,
    export::{export_functions, export_static},
    syscalls::{__user, SYSCALLS},
    statics::StaticRef,
    sysctl::{register_sysctl, Sysctl},
};

// Define a static reference to the syscalls object
static SYSCALLS: StaticRef<SYSCALLS> = export_static! { SYSCALLS };

// Define the sysctl variables for the CPU and GPU voltages
static CPU_VOLTAGE: Sysctl<c_uint> = Sysctl::new("voltage_control/cpu_voltage", 1);
static GPU_VOLTAGE: Sysctl<c_uint> = Sysctl::new("voltage_control/gpu_voltage", 1);

// API function to set the CPU voltage
fn set_cpu_voltage_api(_: __user *mut c_void, voltage: c_uint) -> KernelResult {
    set_cpu_voltage(voltage as u32);
    Ok(0)
}

// API function to set the GPU voltage
fn set_gpu_voltage_api(_: __user *mut c_void, voltage: c_uint) -> KernelResult {
    set_gpu_voltage(voltage as u32);
    Ok(0)
}

// Register the sysctl variables and API functions
fn register_voltage_control() -> Result<(), Error> {
    register_sysctl(&CPU_VOLTAGE)?;
    CPU_VOLTAGE.set_handler(set_cpu_voltage_api)?;

    register_sysctl(&GPU_VOLTAGE)?;
    GPU_VOLTAGE.set_handler(set_gpu_voltage_api)?;

    Ok(())
}

// Export the voltage control functions for use by user-space applications
export_functions! {
    "init_voltage_control" => register_voltage_control,
}
