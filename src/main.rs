#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use rustos::shell::start_shell; // Import the start_shell function
use rustos::fs::{FileSystem, File};  // Correct capitalization for FileSystem
// use crate::FileSystem;
use rustos::keyboard::read_keyboard;
use rustos::println;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use rustos::print;
// defining entry point as kernel_main as the starting point of OS
entry_point!(kernel_main);

// this function is the entry point, since the linker looks for a function
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use x86_64::{VirtAddr};
    use rustos::allocator;
    use rustos::memory::{self, BootInfoFrameAllocator};

    println!("Hello World, {}!!", "from Angshuman");
    

    // Initiations done before the OS bootup
    rustos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {BootInfoFrameAllocator::init(&boot_info.memory_map)};
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let x = Box::new(41);
    println!("heap_value at {:p}", x);
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    // below line defines a conditional compile command
    // whenever test is true the below lines will be compiled and executed

    let mut file_system = FileSystem::new();
    // Start the shell for user input
    start_shell(&mut file_system);
    #[cfg(test)]
    test_main();

    // print statement to check if printing occurse after breakpoint exception instead of crashing
    println!("It did not crash!!");
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
