use std::ffi::CString;

pub fn xhook_register(pathname_regex_str: &str, symbol: &str, new_func: *const u8) {
    let pathname_regex_str = CString::new(pathname_regex_str).unwrap();
    let symbol = CString::new(symbol).unwrap();
    unsafe {
        xhook_sys::xhook_register(
            pathname_regex_str.as_ptr(),
            symbol.as_ptr(),
            new_func as *mut ::std::os::raw::c_void,
            std::ptr::null_mut(),
        );
    }
}

pub fn xhook_refresh(is_async: i32) -> i32 {
    unsafe { xhook_sys::xhook_refresh(is_async) }
}

pub fn xhook_ignore(pathname_regex_str: &str, symbol: Option<&str>) -> i32 {
    let pathname_regex_str = CString::new(pathname_regex_str).unwrap();
    match symbol {
        Some(sym ) => {
            let symbol = CString::new(sym).unwrap();
            unsafe { xhook_sys::xhook_ignore(pathname_regex_str.as_ptr(), symbol.as_ptr()) }
        },
        None => {
            unsafe { xhook_sys::xhook_ignore(pathname_regex_str.as_ptr(), std::ptr::null()) }
        }
    }

}

pub fn xhook_clear() {
    unsafe { xhook_sys::xhook_clear() }
}
pub fn xhook_enable_debug(flag: i32) {
    unsafe { xhook_sys::xhook_enable_debug(flag) }
}
pub fn xhook_enable_sigsegv_protection(flag: i32) {
    unsafe { xhook_sys::xhook_enable_sigsegv_protection(flag) }
}
