#![feature(libc)]

extern crate libc;

#[macro_use]
extern crate redhook;

use libc::{size_t,ssize_t,c_char};

hook! {
    fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: size_t) -> ssize_t => my_readlink {
        if let Ok(path) = std::str::from_utf8(unsafe {std::ffi::CStr::from_ptr(path).to_bytes()}) {
            println!("readlink(\"{}\")", path);
        } else {
            println!("readlink(...)");
        }

        unsafe { real!(readlink)(path, buf, bufsiz) }
    }
}
