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

    #[test]
    fn it_works() {
        println!("{:?}", env!("OUT_DIR"));
        assert_eq!(2 + 2, 4);

        unsafe {
            let c_str = CString::new("hello").unwrap();
            let point = c_str.as_ptr();
            let i = ctags_cli_main(1, point as *mut *mut i8);
        }
    }
}
