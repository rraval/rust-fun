extern crate libc;

use libc::{c_uint, c_char, c_void};
use std::c_str::CString;
use std::mem::{uninitialized};

type VarArgList = *mut c_void;

extern fn rust_variadic(fmt: *const c_char, va_list: VarArgList) {
    unsafe {
        let mut buf = uninitialized::<[c_char, ..128]>();

        vsnprintf(buf.as_mut_ptr(), 128, fmt, va_list);
        println!("{}", CString::new(buf.as_ptr(), false));
    }
}

extern {
    fn callme(cb: unsafe extern fn(*const c_char, ...));

    fn set_shim_callback(cb: extern fn(*const c_char, VarArgList));
    fn shim(num: *const c_char, ...);

    fn vsnprintf(buf: *mut c_char, size: c_uint, fmt: *const c_char, va_list: VarArgList);
}

fn main() {
    unsafe {
        set_shim_callback(rust_variadic);
        callme(shim);
    }
}
