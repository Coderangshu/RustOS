#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use rustos::{fs::FileSystem, shell::start_shell, println};
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

// defining entry point as kernel_main as the starting point of OS
entry_point!(kernel_main);

// this function is the entry point, since the linker looks for a function
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::{VirtAddr};
    use rustos::{allocator, memory::{self, BootInfoFrameAllocator}};

    println!("Welcome to RustOS, {}!!", "from The Rusty Crew");

    // Initiations of Global descriptor table and interrupt handlers
    rustos::init();

    // Virtual memory init and heap allocation inside it
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {BootInfoFrameAllocator::init(&boot_info.memory_map)};
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // Init FileSystem
    let mut file_system = FileSystem::new();

    // Start the shell for user input
    start_shell(&mut file_system);

    // below line defines a conditional compile command
    // whenever test is true the below lines will be compiled and executed
    #[cfg(test)]
    test_main();

    // print statement to check if printing occurse after breakpoint exception instead of crashing
    println!("Kernel is still running!!!!");
    rustos::hlt_loop();
}

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}
