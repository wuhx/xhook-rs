#![feature(c_variadic)]

use std::ffi::CString;
use std::ffi::{c_void, CStr};
use std::os::raw::{c_char, c_int};
use std::slice;
use xhook_rs::*;

#[no_mangle]
pub extern "C" fn my_malloc(size: libc::size_t) -> *mut c_void {
    println!("my malloc, {} bytes!", size);
    unsafe { libc::malloc(size) }
}

#[no_mangle]
pub unsafe extern "C" fn my_open(path: *const c_char, oflag: c_int, args: ...) -> c_int {
    let c_str = unsafe { CStr::from_ptr(path) };
    println!("my_open called: {:?}", c_str.to_str());
    libc::open(path, oflag, args)
}

fn main() {
    xhook_register(".*\\.so$", "open", my_open as *const u8);

    xhook_refresh(0);

    unsafe {
        //should call my_malloc here
        let buf = libc::malloc(10);
        println!("malloc: 10");
    }

    unsafe {
        let input_path = "/data/local/tmp/ADB_RUN/foo.txt";
        let c_input_path = CString::new( input_path).unwrap();
        let c_input_mode = CString::new("rb").unwrap();

        // should call my_open here
        let input_file = libc::fopen(c_input_path.as_ptr(), c_input_mode.as_ptr());
        println!("fopen: {:?}", input_file);


    }
}
