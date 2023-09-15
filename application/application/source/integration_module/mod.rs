// TODO http://blog.asleson.org/2021/02/23/how-to-writing-a-c-shared-library-in-rust/ что здесь за аллокатор.Для чего он нужен?
// TODO https://nadim.computer/posts/2022-02-11-maccatalyst.html
// TODO https://doc.rust-lang.org/nightly/rustc/platform-support.html

// TODO cargo build --release --lib --target aarch64-apple-ios
// TODO cargo build --release --lib --target aarch64-apple-ios-sim
// TODO cargo build --release --lib --target armv7-linux-androideabi

use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn is_equal_to_1(x: c_int) -> bool {
    if x == 1 {
        return true;
    }

    return false;
}

#[no_mangle]
pub extern "C" fn is_x_equal_to_x(
    a: *mut A,
    b: *mut B,
) -> bool {
    let a_ = unsafe { *a };

    let b_ = unsafe { *b };

    if a_.x == b_.x {
        return true;
    }

    return false;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct A {
    pub x: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct B {
    pub x: c_int,
}
