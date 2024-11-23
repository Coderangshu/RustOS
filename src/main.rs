#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rustos::println;
use core::panic::PanicInfo;

// static HELLO: &[u8] = b"Hello World!";


#[no_mangle] // don't mangle the name of this function
// this function is the entry point, since the linker looks for a function
// named `_start` by default
pub extern "C" fn _start() -> ! {
    // 0xb8000 contains the vga buffer, it is a special memory mapped to VGA hardware and
    // contains the contents of the displayed screen

    // let vga_buffer = 0xb8000 as *mut u8;

    // we use unsafe block around all memory writes as otherwise rust compiler won't
    // have allowed this behaviour, as it couldn't prove the raw pointers created do really exist
    // we tell the compiler we are sure about the viability of the pointers called

    // for (i,&byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    // before creating WRITER we need to write all print functions inside the vga module
    // and call from here in this way
    // vga_buffer::print_something();

    // after creating the WRITER we can directly print to screen from anywhere in the code base
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again,\n").unwrap();
    // write!(vga_buffer::WRITER.lock(), "some numbers: {} {}", 43, 1.34343).unwrap();

    // after creating the prinln macro we can simply use it to print to screen
    // as the macro is already part of cargo thus it is globally available no need to import
    println!("Hello World, {}!!", "from Angshuman");

    // Initiations done before the OS bootup
    rustos::init();
    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();

    // trigger a page fault without registering a page fault (this will cause a double fault) (for testing)
    // let ptr = 0xdeadbeaf as *mut u8;
    // unsafe {*ptr = 42;}

    // let ptr = 0x20427c as *mut u8;

    // read from a code page
    // unsafe { let x = *ptr; }
    // println!("read worked");

    // write to a code page
    // unsafe { *ptr = 42; }
    // println!("write worked");

    // causing stack overflow so that a triple page fault occurs
    // fn stack_overflow() {
    //     stack_overflow();
    // }
    // stack_overflow(); // trigger a stack overflow

    // printing base address of the page table
    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    // panic!("sjfdlkfjs"); // code to check if panic! function is working

    // below line defines a conditional compile command
    // whenever test is true the below lines will be compiled and executed
    #[cfg(test)]
    test_main();

    // print statement to check if printing occurse after breakpoint exception instead of crashing
    println!("It did not crash!!");
    rustos::hlt_loop();
    // using print in the loop go into deadlock
    // {
    //     for _ in 0..1000000{
    //         use rustos::print;
    //         print!("-");
    //     }
    // }
}

// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // after implementing the println func in vga_buffer now we
    // can complete this panic function
    println!("{}", info);
    rustos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}
