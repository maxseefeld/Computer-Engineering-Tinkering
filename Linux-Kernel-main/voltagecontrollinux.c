//module to be appended to make menucofnig 
#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/slab.h>
#include <linux/delay.h>
#include <linux/hrtimer.h>
#include <asm/io.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Maxwell Gordon Seefeld");

// Define constants for voltage control
#define MSR_IA32_PERF_CTL 0x199
#define VID_OFFSET 8
#define VID_MASK 0x1F

static uint32_t cpu_model = 0;

// Function to read the model of the CPU
static void get_cpu_model(void) {
    uint32_t eax, ebx, ecx, edx;

    cpuid(0x1, &eax, &ebx, &ecx, &edx);

    cpu_model = (eax & 0xF00) >> 8;
}

// Function to set the voltage of the CPU
static void set_cpu_voltage(uint32_t voltage) {
    uint64_t perf_ctl;

    // Calculate the new performance control value
    perf_ctl = rdmsr(MSR_IA32_PERF_CTL);
    perf_ctl &= ~(VID_MASK << VID_OFFSET);
    perf_ctl |= (voltage << VID_OFFSET);

    // Write the new performance control value
    wrmsr(MSR_IA32_PERF_CTL, perf_ctl);
}

// Initialization function
static int __init voltage_control_init(void) {
    printk(KERN_INFO "Initializing voltage control module\n");

    get_cpu_model();

    if (cpu_model != /* CPU model number */) {
        printk(KERN_ERR "CPU model not supported\n");
        return -EINVAL;
    }

    // Set the CPU voltage to a safe value
    set_cpu_voltage(/* Safe voltage value */);

    return 0;
}

// Cleanup function
static void __exit voltage_control_exit(void) {
    printk(KERN_INFO "Exiting voltage control module\n");
}

// Register the initialization and cleanup functions
module_init(voltage_control_init);
module_exit(voltage_control_exit);
