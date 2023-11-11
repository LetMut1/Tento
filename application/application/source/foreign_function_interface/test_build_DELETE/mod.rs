use libc::c_char;
use libc::c_double;
use libc::c_int;
use libc::c_long;
use libc::c_short;
use libc::c_uchar;
use libc::size_t;
use std::boxed::Box;
use std::default::Default;
use std::error::Error;
use std::ffi::CStr;
use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::forget;
use std::ptr;
use std::ptr::slice_from_raw_parts_mut;
use std::result::Result;
use std::slice;
use std::slice::from_raw_parts;






#[no_mangle]
pub extern "C" fn f1(a: c_int) -> c_int {
    return a;
}

#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_first_step____serialize(a: c_int) -> c_int {
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
                string: string.into_raw()
            }
        )
    )
}

#[no_mangle]
pub extern "C" fn string_deallocate_f2(struct_with_string: *mut StructWithString1) -> () {
    if struct_with_string.is_null() {
        return ();
    }

    let struct_with_string = unsafe {
        Box::from_raw(struct_with_string)
    };

    let string = struct_with_string.string;

    if !string.is_null() {
        let _ = unsafe {
            CString::from_raw(string)
        };
    }

    return ();
}

#[repr(C)]
pub struct StructWithString1 {
    pub is_exist: bool,
    pub string: *mut c_char,
}


#[no_mangle]
pub extern "C" fn string_allocate_f22() -> *mut ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming {
    let string = CString::new("qwerty").unwrap();

    return Box::into_raw(
        Box::new(
            ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming {
                is_exist: true,
                string: string.into_raw()
            }
        )
    )
}

#[no_mangle]
pub extern "C" fn string_deallocate_f22(struct_with_string: *mut ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming) -> () {
    if struct_with_string.is_null() {
        return ();
    }

    let struct_with_string = unsafe {
        Box::from_raw(struct_with_string)
    };

    let string = struct_with_string.string;

    if !string.is_null() {
        let _ = unsafe {
            CString::from_raw(string)
        };
    }

    return ();
}

#[repr(C)]
pub struct ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming {
    pub is_exist: bool,
    pub string: *mut c_char,
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
                        },
                        error: Error2 {
                            is_exist: true,
                        },
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
                },
                error: Error2 {
                    is_exist: false,
                },
            }
        )
    )
}

#[no_mangle]
pub extern "C" fn string_deallocate_f3(struct_with_string: *mut StructWithString2) -> () {
    if struct_with_string.is_null() {
        return ();
    }

    let struct_with_string = unsafe {
        Box::from_raw(struct_with_string)
    };

    let string = struct_with_string.struct_with_string.string;

    if !string.is_null() {
        let _ = unsafe {
            CString::from_raw(string)
        };
    }

    return ();
}


#[repr(C)]
pub struct StructWithString2 {
    pub struct_with_string: StructWithString1,
    pub error: Error2,
}

#[repr(C)]
pub struct Error2 {
    pub is_exist: bool
}




#[no_mangle]
pub extern "C" fn array_slice_f1(pointer_to_first_element_of_registry: *mut c_uchar, registry_length: size_t) -> c_uchar {
    let zero = 0 as c_uchar;

    if pointer_to_first_element_of_registry.is_null() || registry_length == 0 {
        return zero;
    }

    let registry = unsafe {
        slice::from_raw_parts::<u8>(pointer_to_first_element_of_registry, registry_length )
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


#[repr(C)]
#[derive(Clone, Copy)]
pub struct Opaque {
    pub public: bool,
    _private: bool,
}

#[no_mangle]
pub extern "C" fn opaque_f1(opaque: *mut Opaque) -> bool {
    let opaque_ = unsafe { *opaque };

    return opaque_.public == opaque_._private;
}




#[no_mangle]
pub extern "C" fn generic_allocate_f1() -> *mut StructWithGeneric<c_char> {
    return Box::into_raw(
        Box::new(
            StructWithGeneric {
                a: 0 as c_char,
                b: true,
            }
        )
    )
}

#[no_mangle]
pub extern "C" fn generic_deallocate_f1(struct_with_generic: *mut StructWithGeneric<c_char>) -> () {
    if struct_with_generic.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(struct_with_generic)
    };

    return ();
}




#[no_mangle]
pub extern "C" fn generic_allocate_f2() -> *mut StructWithGeneric<*mut c_char> {
    return Box::into_raw(
        Box::new(
            StructWithGeneric {
                a: CString::new("qwerty").unwrap().into_raw(),
                b: true,
            }
        )
    )
}

#[no_mangle]
pub extern "C" fn generic_deallocate_f2(struct_with_generic: *mut StructWithGeneric<*mut c_char>) -> () {
    if struct_with_generic.is_null() {
        return ();
    }

    let struct_with_generic = unsafe {
        Box::from_raw(struct_with_generic)
    };

    let string = struct_with_generic.a;

    if !string.is_null() {
        let _ = unsafe {
            CString::from_raw(string)
        };
    }

    return ();
}

#[repr(C)]
pub struct StructWithGeneric<T> {
    pub a: T,
    pub b: bool,
}






#[repr(C)]
#[derive(Clone, Copy)]
pub struct Main1 {
    pub nested1: Nested1,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Main2 {
    pub nested1: Nested1,
    pub nested2: Nested2,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Nested1 {
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub string: *mut c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Nested2 {
    pub a: bool,
}

#[no_mangle]
pub extern "C" fn main_nested_allocate_f1() -> *mut Main1 {
    return Box::into_raw(
        Box::new(
            Main1 { nested1: Nested1 { a: true, b: false, c: true, string: CString::new("qwerty").unwrap().into_raw() } }
        )
    )
}

#[no_mangle]
pub extern "C" fn main_nested_deallocate_f1(main: *mut Main1) -> () {
    if main.is_null() {
        return ();
    }

    let main = unsafe {
        Box::from_raw(main)
    };

    if !main.nested1.string.is_null() {
        let _ = unsafe {
            CString::from_raw(main.nested1.string)
        };
    }

    return ();
}

#[no_mangle]
pub extern "C" fn main_nested_allocate_f2() -> *mut Main2 {
    return Box::into_raw(
        Box::new(
            Main2 {
                nested1: Nested1 {
                    a: true,
                    b: false,
                    c: true,
                    string: CString::new("qwerty_12334_qwertyu").unwrap().into_raw()
                },
                nested2: Nested2 {
                    a: true
                }
            }
        )
    )
}

#[no_mangle]
pub extern "C" fn main_nested_deallocate_f2(main: *mut Main2) -> () {
    if main.is_null() {
        return ();
    }

    let main = unsafe {
        Box::from_raw(main)
    };

    if !main.nested1.string.is_null() {
        let _ = unsafe {
            CString::from_raw(main.nested1.string)
        };
    }

    return ();
}
