extern crate libc;

use libc::{c_int, c_void};

type VarArgList = *mut c_void;

extern fn rust_variadic(num: c_int, va_list: VarArgList) {
    unsafe {
        println!("got {} args", num);

        for _ in range(0, num) {
            println!("{}", va_arg_int(va_list));
        }
    }
}

extern {
    fn callme(cb: unsafe extern fn(c_int, ...));

    fn set_shim_callback(cb: extern fn(c_int, VarArgList));
    fn shim(num: c_int, ...);

    fn va_arg_int(va_list: VarArgList) -> c_int;
}

fn main() {
    unsafe {
        set_shim_callback(rust_variadic);
        callme(shim);
    }
}
