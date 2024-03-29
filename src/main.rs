#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ossa::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ossa::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    ossa::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    ossa::test_panic_handler(info)
}
