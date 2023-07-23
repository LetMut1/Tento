use std::os::raw::c_int;

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
    let a_ = unsafe {
        *a
    };

    let b_ = unsafe {
        *b
    };

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