use libc::c_char;
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