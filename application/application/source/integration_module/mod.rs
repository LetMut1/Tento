// TODO cargo build --release --lib --target aarch64-apple-ios
// TODO cargo build --release --lib --target aarch64-apple-ios-sim
// TODO cargo build --release --lib --target armv7-linux-androideabi


// TODO Везде в unsafe функциях (slice::from_raw_parts, ...) ядра посмотреть требования к параметрам и попробовать сделать проверку.

use libc::c_int;
use libc::c_double;
use libc::c_uchar;
use libc::c_char;
use std::ffi::CString;
use std::slice;
use libc::size_t;
use std::ptr;

#[no_mangle]
pub extern "C" fn f1(a: c_int) -> c_int {
    return a;
}

#[no_mangle]
pub extern "C" fn f2(a: bool) -> bool {
    return a;
}

#[no_mangle]
pub extern "C" fn f3(a: c_double) -> c_double {
    return a;
}

#[no_mangle]
pub extern "C" fn f4(a: A) -> c_int {
    return a.a;
}

#[no_mangle]
pub extern "C" fn f5(a: A) -> B {
    return B { a: a.a };
}

#[no_mangle]
pub extern "C" fn f6(a: A, b: B) -> bool {
    return a.a == b.a;
}

#[no_mangle]
pub extern "C" fn f7(a: *const c_int,) -> c_int {
    let a_ = unsafe { *a };

    return a_;
}

#[no_mangle]
pub extern "C" fn f8(a: *mut c_int,) -> c_int {
    let a_ = unsafe { *a };

    return a_;
}

#[no_mangle]
pub extern "C" fn f9(a: *const A,) -> c_int {
    let a_ = unsafe { *a };

    return a_.a;
}

#[no_mangle]
pub extern "C" fn f10(a: *mut A,) -> c_int {
    let a_ = unsafe { *a };

    return a_.a;
}

#[no_mangle]
pub extern "C" fn f11(a: *mut A, b: *mut B,) -> bool {
    let a_ = unsafe { *a };

    let b_ = unsafe { *b };

    if a_.a == b_.a {
        return true;
    }

    return false;
}

#[no_mangle]
pub extern "C" fn f12(a: *mut C, b: c_int,) -> C {
    let mut a_ = unsafe { *a };

    a_.a = b;

    return a_;
}

#[no_mangle]
pub extern "C" fn f13(a: *mut C, b: bool,) -> () {
    let mut a_ = unsafe { *a };

    a_.b = b;

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct A {
    pub a: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct B {
    pub a: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C {
    pub a: c_int,
    pub b: bool,
}




#[no_mangle]
pub extern "C" fn string_allocate_f1() -> *mut c_char {
    let string = CString::new("qwerty").unwrap();

    return string.into_raw();
}

#[no_mangle]
pub extern "C" fn string_deallocate_f1(string: *mut c_char) -> () {
    if string.is_null() {
        return ();
    }

    let _ = unsafe {
        CString::from_raw(string)
    };

    return ();
}




#[no_mangle]
pub extern "C" fn string_allocate_f2() -> *mut StructWithString1 {
    let string = CString::new("qwerty").unwrap();

    return Box::into_raw(
        Box::new(
            StructWithString1 {
                is_exist: true,
                string: string.into_raw(),
                _private: ()
            }
        )
    )
}

#[no_mangle]
pub extern "C" fn string_deallocate_f2(struct_with_string: *mut StructWithString1) -> () {
    if struct_with_string.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(struct_with_string)
    };

    return ();
}

#[repr(C)]
pub struct StructWithString1 {
    pub is_exist: bool,
    pub string: *mut c_char,
    _private: (),                                   // TODO TODO TODO TODO TODO PhantomData можно ли?
}




#[no_mangle]
pub extern "C" fn string_allocate_f3() -> *mut StructWithString2 {
    let string = match CString::new("qwerty") {
        Ok(string_) => string_,
        Err(_) => {
            return Box::into_raw(
                Box::new(
                    StructWithString2 {
                        struct_with_string: StructWithString1 {
                            is_exist: false,


                            string: ptr::null_mut(),
                            _private: ()
                        },
                        error: Error {
                            is_exist: true,
                        },
                        _private: ()
                    }
                )
            )
        }
    };

    return Box::into_raw(
        Box::new(
            StructWithString2 {
                struct_with_string: StructWithString1 {
                    is_exist: true,
                    string: string.into_raw(),
                    _private: ()
                },
                error: Error {
                    is_exist: false,
                },
                _private: ()
            }
        )
    )
}

#[no_mangle]
pub extern "C" fn string_deallocate_f3(struct_with_string: *mut StructWithString2) -> () {
    if struct_with_string.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(struct_with_string)
    };

    return ();
}

#[repr(C)]
pub struct StructWithString2 {
    pub struct_with_string: StructWithString1,
    pub error: Error,
    _private: ()
}

#[repr(C)]
pub struct Error {
    is_exist: bool
}




#[no_mangle]
pub extern "C" fn array_slice_f1(pointer_to_first_element_of_registry: *mut c_uchar, registry_length: size_t) -> c_uchar {
    let zero = 0 as c_uchar;

    if pointer_to_first_element_of_registry.is_null() || registry_length == 0 {
        return zero;
    }

    let registry = unsafe {
        slice::from_raw_parts::<u8>(pointer_to_first_element_of_registry, registry_length as usize)
    };

    if registry.len() == 0 {
        return zero;
    }

    let element = match registry.last() {
        Some(element_) => element_,
        None => {
            return zero;
        }
    };

    return *element as c_uchar;
}