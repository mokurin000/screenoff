#![no_std]
#![no_main]

use core::ffi::{c_uint, c_void};

windows_link::link!(
    "user32.dll" "system"
    fn PostMessageA(
        handle:*const c_void,
        msg: c_uint,
        w_param: usize,
        l_param: isize,
    ) -> isize
);

fn screen_off() {
    unsafe {
        PostMessageA(-1isize as _, 0x112, 0xF170, 2);
    }
}

#[unsafe(no_mangle)]
extern "system" fn main() -> i32 {
    screen_off();
    0
}
