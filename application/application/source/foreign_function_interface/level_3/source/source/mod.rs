use libc::c_char;
use libc::c_uchar;
use libc::c_ushort;
use std::ffi::CStr;
use std::ffi::CString;
// level_1---------------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn get_number() -> c_char {
    return 69;
}
#[no_mangle]
pub extern "C" fn is_goyda() -> bool {
    return true;
}
#[no_mangle]
pub extern "C" fn convert_to_bool(number: c_char) -> bool {
    return number > 0;
}
// level_1---------------------------------------------------------------------------------
// level_2---------------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn check_for_nigger_word(pointer_to_string: *const c_char) -> bool {
    let string = unsafe {
        CStr::from_ptr(pointer_to_string)
    }
    .to_str()
    .unwrap();
    return string == "Nigger";
}
#[no_mangle]
pub extern "C" fn get_nigger_word__allocate(quantity: c_uchar) -> *mut c_char {
    let string = if quantity == 0 {
        "Zero Niggers... it is a joke?!".to_string()
    } else {
        "Nigger".repeat(quantity as usize)
    };
    return CString::new(string).unwrap().into_raw();
}
#[no_mangle]
pub extern "C" fn get_nigger_word__deallocate(pointer_to_string: *mut c_char) -> () {
    let _ = unsafe {
        CString::from_raw(pointer_to_string)
    };
    return ();
}
// level_2---------------------------------------------------------------------------------
// level_3---------------------------------------------------------------------------------
#[repr(C)]
pub struct A {
    a: c_uchar,
    b: c_uchar,
}
#[no_mangle]
pub extern "C" fn get_a(a: c_uchar) -> A {
    return A {
        a,
        b: a,
    };
}
#[no_mangle]
pub extern "C" fn calculate_a_sum(a: A) -> c_ushort {
    return (a.a as u16) + (a.b as u16);
}
#[repr(C)]
pub struct B {
    a: c_uchar,
    b: c_uchar,
}
#[no_mangle]
pub extern "C" fn get_b__allocate(a: c_uchar) -> *mut B {
    return Box::into_raw(
        Box::new(
            B {
                a,
                b: a,
            },
        ),
    );
}
#[no_mangle]
pub extern "C" fn get_b__deallocate(b: *mut B) -> () {
    let _ = unsafe {
        Box::from_raw(b)
    };
    return ();
}
#[no_mangle]
pub extern "C" fn calculate_b_sum(b: *mut B) -> c_ushort {
    let b = unsafe {
        &*b
    };
    return (b.a as u16) + (b.b as u16);
}
#[repr(C)]
pub struct C {
    a: c_uchar,
    string_pointer: *mut c_char,
}
#[no_mangle]
pub extern "C" fn get_c__allocate() -> *mut C {
    return Box::into_raw(
        Box::new(
            C {
                a: 1,
                string_pointer: CString::new("Nigger").unwrap().into_raw(),
            },
        ),
    );
}
#[no_mangle]
pub extern "C" fn get_c__deallocate(c: *mut C) -> () {
    let c = unsafe {
        Box::from_raw(c)
    };
    let _ = unsafe {
        CString::from_raw(c.string_pointer)
    };
    return ();
}
#[no_mangle]
pub extern "C" fn is_c__1_nigger___pointer(c: *mut C) -> bool {
    let c = unsafe {
        &*c
    };
    let string = unsafe {
        CStr::from_ptr(c.string_pointer)
    }
    .to_str()
    .unwrap();
    return c.a == 1 && string == "Nigger";
}
#[no_mangle]
pub extern "C" fn is_c__1_nigger___full_struct(c: C) -> bool {
    let string = unsafe {
        CStr::from_ptr(c.string_pointer)
    }
    .to_str()
    .unwrap();
    return c.a == 1 && string == "Nigger";
}