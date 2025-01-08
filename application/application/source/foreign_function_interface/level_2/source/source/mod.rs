use libc::c_char;
use libc::c_uchar;
use std::ffi::CStr;
use std::ffi::CString;
// level_1---------------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn get_number() -> c_char {
    return 69;
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