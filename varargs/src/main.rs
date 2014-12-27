extern crate libc;

use libc::types::common::c95::c_void;

type VarArgList = *mut c_void;

extern fn rust_variadic(num: i32, va_list: VarArgList) {
    unsafe {
        println!("got {} args", num);

        let mut arg = 0i32;
        for _ in range(0, num) {
            va_arg_int(va_list, &mut arg);
            println!("{}", arg);
        }
    }
}

extern {
    fn callme(cb: unsafe extern fn(i32, ...));

    fn set_shim_callback(cb: extern fn(i32, VarArgList));
    fn shim(num: i32, ...);

    fn va_arg_int(va_list: VarArgList, arg: *mut i32);
}

fn main() {
    unsafe {
        set_shim_callback(rust_variadic);
        callme(shim);
    }
}
