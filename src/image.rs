use ffi;
use VipsBandFormat;
use std::error::Error;
use std::os::raw::c_char;
use std::ptr::null;
use common::VipsSize;
use common::VipsKernel;
use std::os::raw::c_void;
use std::ffi::CString;
use common::current_error;


pub struct VipsImage {
    pub c: *mut ffi::VipsImage
}

impl Drop for VipsImage {
    fn drop(&mut self) {
        unsafe {
            ffi::g_object_unref(self.c as *mut c_void);
        }
    }
}

pub struct VipsBufferImage<'a> {
    pub buf: &'a [u8],
    pub c: *mut ffi::VipsImage
}

impl<'a> Drop for VipsBufferImage<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::g_object_unref(self.c as *mut c_void);
        }
    }
}

impl VipsImage {
    pub fn new() -> VipsImage {
        unsafe {
            VipsImage {
                c: ffi::vips_image_new()
            }
        }
    }

    pub fn new_memory() -> VipsImage {
        unsafe {
            VipsImage {
                c: ffi::vips_image_new_memory()
            }
        }
    }


    pub fn new_from_file<S: Into<Vec<u8>>>(path: S) -> Result<VipsImage, Box<Error>> {
        let path = CString::new(path)?;

        unsafe {
            let img = ffi::vips_image_new_from_file(path.as_ptr(), null() as *const c_char);
            if img.is_null() {
                Err(current_error().into())
            } else {
                Ok(VipsImage {
                    c: img
                })
            }
        }
    }
}

pub trait VipsImageTrait {
    fn c_data(&mut self) -> *mut ffi::VipsImage;

    fn thumbnail(&mut self, width:u32, height:u32, size:VipsSize) -> VipsImage {
        let mut out = VipsImage::new_memory();
        unsafe {
            ffi::vips_thumbnail_image(self.c_data(), &mut out.c, width as i32, "height\0".as_ptr(), height as i32, "size\0".as_ptr(), size, null() as *const c_char);
        }
        out
    }

    fn resize(&self, scale:f64, vscale:Option<f64>, kernel:Option<VipsKernel>) { // block shrink + lanczos3
        unimplemented!();
    }

    // low-level
    // default: 2 * 1D lanczos3 (not recommended for shrink factor > 3)
    // or other kernels
    fn reduce(&self, hshrink:f64, vshrink:f64, kernel:Option<VipsKernel>, centre:Option<bool>) -> VipsImage {
        unimplemented!();
//        unsafe {
//            ffi::vips_reduce(self.c, , )
//        }
    }

    fn shrink() { // simple average of nxn -> 1/n size
        unimplemented!();
    }

    fn jpegsave<S:Into<Vec<u8>>>(&mut self, path: S) -> Result<(),Box<Error>> {
        let path = CString::new(path)?;
        let ret = unsafe { ffi::vips_jpegsave(self.c_data(), path.as_ptr(), null() as *const c_char) };
        match ret {
            0 => Ok(()),
            _ => Err(current_error().into()),
        }
    }

    fn write_to_file<S:Into<Vec<u8>>>(&mut self, path: S) -> Result<(),Box<Error>> {
        let path = CString::new(path)?;
        let ret = unsafe { ffi::vips_image_write_to_file(self.c_data(), path.as_ptr(), null() as *const c_char) };
        match ret {
            0 => Ok(()),
            _ => Err(current_error().into()),
        }
    }
}

impl VipsImageTrait for VipsImage {
    fn c_data(&mut self) -> *mut ffi::VipsImage {
        self.c
    }
}

impl<'a> VipsImageTrait for VipsBufferImage<'a> {
    fn c_data(&mut self) -> *mut ffi::VipsImage {
        self.c
    }
}

impl<'a> VipsBufferImage<'a> {
    pub fn new_from_memory(buf: &'a [u8], width: u32, height: u32, bytes_per_pixel: u32, format: VipsBandFormat) -> VipsBufferImage<'a> {
        unsafe {
            let img = ffi::vips_image_new_from_memory(buf.as_ptr() as *const c_void, buf.len(), width as i32, height as i32, bytes_per_pixel as i32, format as i32);
            VipsBufferImage {
                buf,
                c: img
            }
        }
    }
}
