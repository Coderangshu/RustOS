#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

// --------------------------------------- Testing Part ------------------------------------------------------------

#![feature(custom_test_frameworks)] // Custom test framework provided by Rust
#![test_runner(crate::test_runner)]
// The custom test frameworks feature generates a main function that calls test_runner,
// but this function is ignored because we use the #[no_main] attribute and provide our own entry point _start
// To fix this, we first need to change the name of the generated function to something different than main through
// the reexport_test_harness_main attribute. Then we can call the renamed function from our _start function
#![reexport_test_harness_main = "test_main"]

// Test framework code
// tests is passed as argument to the test_runner, it contains all the test cases
// which are the reference of trivial_assertion (test cases), these are then executed
// by the test_runner
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

// --------------------------------------- Testing Part ------------------------------------------------------------

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // after implementing the println func in vga_buffer now we
    // can complete this panic function
    println!("{}", info);
    loop {}
}

// a module to handle VGA hardware supported printing
mod vga_buffer;

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

    // panic!("sjfdlkfjs"); // code to check if panic! function is working

    // below line defines a conditional compile command
    // whenever test is true the below lines will be compiled and executed
    #[cfg(test)]
    test_main();

    loop {}
}
