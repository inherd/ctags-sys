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

    #[test]
    fn it_works() {
        let mut p = Box::into_raw(Box::new("src/ffi.rs".to_string())) as *mut i8;

        unsafe {
            let _i = ctags_cli_main(1, &mut p as _);
        }

        assert_eq!(1, 1);
    }
}
