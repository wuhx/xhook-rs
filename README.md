
# xHook-rs

xHook bindings for Rust


## example usage
```rust

use xhook_rs::*;

#[no_mangle]
pub extern "C" fn my_malloc(size: libc::size_t) -> *mut c_void {
    println!("my malloc, {} bytes!", size);
    unsafe { libc::malloc(size) }
}


fn main() {
    xhook_register(".*\\.so$", "malloc", my_malloc as *const u8);

    xhook_refresh(0);
    unsafe {
        //should call my_malloc here
        let buf = libc::malloc(10);
        println!("malloc: 10");
    }
} 
    
```


## more info 
https://github.com/iqiyi/xHook
