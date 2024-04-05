#![no_std]
#![no_main]

use core::panic::PanicInfo;
use phil_opp_rust_os::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failure);
    should_also_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failure);
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

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

fn should_also_fail() {
    serial_print!("should_panic::should_also_fail...\t");
    assert_eq!(0, 1);
}