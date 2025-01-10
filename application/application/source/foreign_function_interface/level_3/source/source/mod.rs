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
pub extern "C" fn calculate_scd_sum(b: *mut B) -> c_ushort {
    let b = unsafe {
        Box::from_raw(b)
    };
    return (b.a as u16) + (b.b as u16);
}
#[repr(C)]
#[derive(PartialEq, Eq)]
pub enum C {
    One,
    Two,
}
#[no_mangle]
pub extern "C" fn get_c(number: c_uchar) -> C {
    return if number <= 127 {
        C::One
    } else {
        C::Two
    };
}
#[no_mangle]
pub extern "C" fn is_c_one(c: C) -> bool {
    return c == C::One;
}
#[repr(C)]
#[derive(PartialEq, Eq)]
pub enum D {
    Three,
    Four,
}
#[no_mangle]
pub extern "C" fn get_d__allocate(number: c_uchar) -> *mut D {
    let d = if number <= 127 {
        D::Three
    } else {
        D::Four
    };
    return Box::into_raw(Box::new(d));
}
#[no_mangle]
pub extern "C" fn get_d__deallocate(d: *mut D) -> () {
    let _ = unsafe {
        Box::from_raw(d)
    };
    return ();
}
#[no_mangle]
pub extern "C" fn is_d_one(d: *mut D) -> bool {
    let d = unsafe {
        Box::from_raw(d)
    };
    return d.as_ref() == &D::Three;
}
#[repr(C)]
pub enum E {
    Five {
        a: c_char,
        b_pointer_to_string: *mut c_char
    },
    Six {
        a: bool,
    },
}
#[no_mangle]
pub extern "C" fn get_e__allocate_1() -> *mut E {
    return Box::into_raw(
        Box::new(
            E::Five {
                a: 69,
                b_pointer_to_string: CString::new("Nigger").unwrap().into_raw(),
            },
        ),
    );
}
#[no_mangle]
pub extern "C" fn get_e__allocate_2() -> *mut E {
    return Box::into_raw(
        Box::new(
            E::Six {
                a: true,
            },
        )
    );
}
#[no_mangle]
pub extern "C" fn get_e__deallocate(e: *mut E) -> () {
    let e = unsafe {
        Box::from_raw(e)
    };
    match *e {
        E::Five {
            a: _,
            b_pointer_to_string,
        } => {
            let _ = unsafe {
                CString::from_raw(b_pointer_to_string)
            };
        }
        _ => {}
    }
    return ();
}
#[no_mangle]
pub extern "C" fn is_e_one_69_nigger(e: *mut E) -> bool {
    let e = unsafe {
        Box::from_raw(e)
    };
    match *e {
        E::Five {
            a,
            b_pointer_to_string,
        } => {
            let b_pointer_to_string_ = unsafe {
                CStr::from_ptr(b_pointer_to_string).to_str().unwrap()
            };
            return a == 69 && b_pointer_to_string_ == "Nigger";
        }
        _ => {}
    }
    return true;
}