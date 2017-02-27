#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/ffi.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webp() {
        use std::io::prelude::*;
        use std::fs::File;

        let mut width = 0;
        let mut height = 0;
        let mut buf = Vec::new();
        let len = File::open("1.webp").unwrap().read_to_end(&mut buf).unwrap();
        unsafe { WebPGetInfo(buf.as_ptr(), len, &mut width, &mut height); }
        assert!(width == 1290);
        assert!(height == 996);
    }
}
