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

        let mut params: Vec<&str> = vec![
            "-p",
            "src/ffi.rs"
        ];
        let mut parameters: Vec<*mut i8> =
            params
                .iter()
                .map(|s| {
                    "src/ffi.rs".to_string().as_mut_ptr() as *mut i8
                }).collect();

        unsafe {
            let i = ctags_cli_main(1, parameters.as_mut_ptr());
        }
    }
}
