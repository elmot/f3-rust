#![no_std]

//use core::fmt::{Write, self};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
//    let mut host_stderr = HStderr::new();

    // logs "panicked at '$reason', src/main.rs:27:4" to the host stderr
//    writeln!(host_stderr, "{}", info).ok();

    loop {}
}

//fn main() {
//    println!("Hello, world!");
//}

extern "C" {
    fn outputStr(s: *const u8, len: usize);
}

#[no_mangle]
pub extern "C" fn foo() {
    let s = "Hello, Embedded World";

    unsafe { outputStr(s.as_ptr(), s.len()); }
}