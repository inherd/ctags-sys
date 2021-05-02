pub mod bindgend;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
mod ffi;

pub use self::ffi::*;

#[cfg(test)]
mod tests {
    extern crate libc;
    use crate::{ctags_cli_main};
    use std::ffi::CString;
    use std::os::raw::c_char;

    #[test]
    fn it_works() {
        println!("{:?}", env!("OUT_DIR"));
        assert_eq!(2 + 2, 4);

        let mut params: Vec<String> = vec![
            "-p".to_string(),
            "src/ffi.rs".to_string()
        ];

        // let args = params.map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
        // // convert the strings to raw pointers
        // let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();

        // let result = CString::new("hello").unwrap().as_ptr();
        //
        // let mut parameters: Vec<*mut i8> =
        //     params
        //         .iter()
        //         .map(|s| {
        //             // "src/ffi.rs".to_string().as_mut_ptr() as *mut i8
        //             CString::new(s.to_string()).unwrap().as_ptr() as *mut i8
        //         }).collect();

        // let mut p = Box::into_raw(Box::new("-p src/ffi.rs")) as *mut i8;

        let mut parameters: Vec<*mut i8> = params.iter().map(|s| {
            Box::into_raw(Box::new(s.to_string())) as *mut i8
        }).collect();

        unsafe {
            let i = ctags_cli_main(1, parameters.as_mut_ptr());
        }
    }
}
