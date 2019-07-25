#![no_std]

mod system;

extern "C" {
    fn HAL_GPIO_TogglePin(GPIOx: *const () /*todo structure type*/, GPIO_Pin: u16);
    fn HAL_Delay(ms: u32);
    fn outputStr(s: *const u8, len: usize); //todo remove?
    static RUST_GPIOE: u32;
}

#[no_mangle]
pub extern "C" fn foo() {
//    let s = "Hello, Embedded World";

    unsafe {
//        outputStr(s.as_ptr(), s.len());
        HAL_GPIO_TogglePin(RUST_GPIOE as * const (), 0x2000);
        HAL_Delay(350);
    }
}