use libc::c_char;
// level_1---------------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn get_number() -> c_char {
    return 69;
}