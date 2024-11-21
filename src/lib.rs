#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)] // to allow x86_interrupt to run in our OS
#![feature(custom_test_frameworks)] // Custom test framework provided by Rust
#![test_runner(crate::test_runner)]
// The custom test frameworks feature generates a main function that calls test_runner,
// but this function is ignored because we use the #[no_main] attribute and provide our own entry point _start
// To fix this, we first need to change the name of the generated function to something different than main through
// the reexport_test_harness_main attribute. Then we can call the renamed function from our _start function
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;

// Code block for the Interrupt Descriptor Table init and others
pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe {interrupts::PICS.lock().initialize()};
    x86_64::instructions::interrupts::enable();
}

// function to prevent continuous loop,  hlt puts CPU to sleed until next interrupt
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

// Testing code blocks
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t",core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

// Test framework code
// tests is passed as argument to the test_runner, it contains all the test cases
// which are the reference of trivial_assertion (test cases), these are then executed
// by the test_runner
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

// Test exit code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init(); // init initiates the IDT when test environment is started
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

// a breakpoint exception testing test case
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3(); // invoke a breakpoint exception
}