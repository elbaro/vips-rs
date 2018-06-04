use ::ffi;
use VipsImage;
use std::os::raw::c_void;
use std::os::raw::c_int;
use std::os::raw::c_char;
use std::ptr::null;
use std::error::Error;

pub trait VipsBuffer {
    fn thumbnail(&self, width:u32, height:u32) -> Result<VipsImage, Box<Error>>;
}

impl<'a> VipsBuffer for &'a [u8] {
    fn thumbnail(&self, width:u32, height:u32) -> Result<VipsImage, Box<Error>> {
        unsafe {
            let mut out = VipsImage::new_memory()?;
            ffi::vips_thumbnail_buffer(self.as_ptr() as *mut c_void, self.len(), &mut out.c, width as c_int, "height\0".as_ptr(), height as c_int, "size\0".as_ptr(), ffi::VipsSize::VIPS_SIZE_FORCE, null() as *const c_char);
            Ok(out)
        }
    }

    // pub fn jpegload(&self) -> Result<VipsImage, Box<Error>> {
    //     let mut out = VipsImage::new_memory()?;
    //     unsafe {
    //         ffi::vips_jpegload_buffer(self.as_mut_ptr() as *mut c_void, buf.len(), &mut out.c);
    //     }
    //     Ok(out)
    // }
}
