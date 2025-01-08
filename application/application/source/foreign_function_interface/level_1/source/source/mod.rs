use libc::c_char;

#[no_mangle]
pub extern "C" fn get_number() -> c_char {
    return 69;
}