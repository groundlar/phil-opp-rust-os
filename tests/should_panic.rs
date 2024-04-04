#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use phil_opp_rust_os::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

// TODO this only allows running a single test,
// e.g. only one of `should_fail` and `should_also_fail`
// will be run.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("[test did not panic]");
        exit_qemu(QemuExitCode::Failure);
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[test_case]
fn should_also_fail() {
    serial_print!("should_panic::should_also_fail...\t");
    assert_eq!(0, 1);
}
