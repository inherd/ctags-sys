pub mod bindgend;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
mod ffi;

pub use self::ffi::*;

#[cfg(test)]
mod tests {
    use crate::{ctags_cli_main};
    use std::ffi::CString;
    use std::os::raw::c_char;
    use std::ptr;

    #[test]
    fn it_works() {
        let args: Vec<CString> = vec![
            CString::new("--verbose").unwrap(),
            CString::new("src/ffi.rs").unwrap()
        ];

        // ref to https://cxx.rs/binding/rawptr.html
        let argc = args.len();
        let mut argv: Vec<*mut c_char> = Vec::with_capacity(argc + 1);
        for arg in &args {
            argv.push(arg.as_ptr() as *mut c_char);
        }
        argv.push(ptr::null_mut()); // Nul terminator.

        unsafe {
            let i = ctags_cli_main(1, argv.as_mut_ptr());
        }

        assert_eq!(1, 1);
    }
}
