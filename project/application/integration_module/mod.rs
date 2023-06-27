use std::os::raw::c_long;

// TODO DELETE TODO DELETE TODO DELETE TODO DELETE TODO DELETE
#[no_mangle]
pub extern "C" fn test1(x: c_long) -> bool {
    if x > 1 {
        return true;
    }

    return false;
}
