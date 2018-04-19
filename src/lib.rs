#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]
extern crate vips_sys as ffi;
#[macro_use]
extern crate lazy_static;

use std::os::raw::c_void;

// re-exports
mod common;
pub use common::*;

mod instance;
pub use instance::VipsInstance;

mod image;
pub use image::{VipsImage, VipsBufferImage, VipsImageTrait};

pub fn thumbnail_buffer(data: &[u8], width:u32, height:u32) -> VipsImage {
//        ffi::vips_thumbnail_buffer(data.as_ptr() as *const c_void, data.len(), width, )
    unimplemented!();
}

pub fn jpegload_buffer(buf: &mut [u8]) -> VipsImage {
    let mut out = VipsImage::new_memory();
    unsafe {
        ffi::vips_jpegload_buffer(buf.as_mut_ptr() as *mut c_void, buf.len(), &mut out.c);
    }
    out
}


#[cfg(test)]
mod tests {
    use ::*;
    #[test]
    fn it_works() {
        let instance = VipsInstance::new("app_test", true).unwrap();
        let mut img = VipsImage::new_from_file("kodim01.png").unwrap();
        let mut img = img.thumbnail(123, 234, VipsSize::VIPS_SIZE_FORCE);
        img.write_to_file("kodim01_123x234.png").unwrap();
    }

    #[test]
    fn unique_instance() {
        assert!(VipsInstance::new("app_test", true).is_err());
    }
}
