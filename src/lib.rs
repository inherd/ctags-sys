pub mod bindgend;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(clippy::all)]
mod ffi;


pub use self::ffi::*;

#[cfg(test)]
mod tests {
    use crate::{clock};

    #[test]
    fn it_works() {
        println!("{:?}", env!("OUT_DIR"));
        assert_eq!(2 + 2, 4);

        unsafe {
            let i = clock();
            println!("{:?}", i);
        }
    }
}
