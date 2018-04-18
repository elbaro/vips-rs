#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]
extern crate vips_sys as ffi;
use std::ffi::CString;
use std::os::raw::c_void;
use std::error::Error;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr::null;
use std::os::raw::c_int;


pub struct VipsInstance {

}

impl VipsInstance {
    pub fn new(name:&str, leak_test:bool) -> VipsInstance {
        let c = CString::new(name).unwrap();
        unsafe {
            ffi::vips_init(c.as_ptr());
            if leak_test {
                ffi::vips_leak_set(true as c_int);
            }
        }
        VipsInstance {}
    }
}

impl Drop for VipsInstance {
    fn drop(&mut self) {
        unsafe {
            ffi::vips_shutdown();
        }
    }
}

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

pub fn current_error() -> String {
    let msg = unsafe {
        CStr::from_ptr(ffi::vips_error_buffer())
    };
    msg.to_str().unwrap().to_string()
}

pub enum VipsSize {
    VIPS_SIZE_BOTH = 0,
    VIPS_SIZE_UP = 1,
    VIPS_SIZE_DOWN = 2,
    VIPS_SIZE_FORCE = 3,
    VIPS_SIZE_LAST = 4,
}

pub enum VipsKernel {
    VIPS_KERNEL_NEAREST = 0,
    VIPS_KERNEL_LINEAR = 1,
    VIPS_KERNEL_CUBIC = 2,
    VIPS_KERNEL_LANCZOS2 = 3,
    VIPS_KERNEL_LANCZOS3 = 4,
    VIPS_KERNEL_LAST = 5,
}

pub struct VipsImage {
    c: *mut ffi::VipsImage
}

impl Drop for VipsImage {
    fn drop(&mut self) {
        unsafe {
//            println!("free! {:?}", self.c);
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

    pub fn new_from_memory(data: &[u8], width: u32, height: u32, bytes_per_pixel: u32, format: VipsBandFormat) -> VipsImage {
        unsafe {
            let img = ffi::vips_image_new_from_memory(data.as_ptr() as *const std::os::raw::c_void, data.len(), width as i32, height as i32, bytes_per_pixel as i32, format as i32);
            VipsImage {
                c: img
            }
        }
    }

    pub fn new_from_file<S:Into<Vec<u8>>>(path:S) -> Result<VipsImage, Box<Error>> {
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

    pub fn thumbnail(&mut self, width:u32, height:u32, size:VipsSize) -> VipsImage {
        let mut out = VipsImage::new_memory();
        unsafe {
            ffi::vips_thumbnail_image(self.c, &mut out.c, width as i32, "height\0".as_ptr(), height as i32, "size\0".as_ptr(), size, null() as *const c_char);
        }
        out
    }

    pub fn resize(&self, scale:f64, vscale:Option<f64>, kernel:Option<VipsKernel>) { // block shrink + lanczos3
        unimplemented!();
    }


    // low-level
    // default: 2 * 1D lanczos3 (not recommended for shrink factor > 3)
    // or other kernels
    pub fn reduce(&self, hshrink:f64, vshrink:f64, kernel:Option<VipsKernel>, centre:Option<bool>) -> VipsImage {
        unimplemented!();
//        unsafe {
//            ffi::vips_reduce(self.c, , )
//        }
    }

    pub fn shrink() { // simple average of nxn -> 1/n size
        unimplemented!();
    }

    pub fn jpegsave<S:Into<Vec<u8>>>(&mut self, path: S) -> Result<(),Box<Error>> {
        let path = CString::new(path)?;
        let ret = unsafe { ffi::vips_jpegsave(self.c, path.as_ptr(), null() as *const c_char) };
        match ret {
            0 => Ok(()),
            _ => Err(current_error().into()),
        }
    }

    pub fn write_to_file<S:Into<Vec<u8>>>(&mut self, path: S) -> Result<(),Box<Error>> {
        let path = CString::new(path)?;
        let ret = unsafe { ffi::vips_image_write_to_file(self.c, path.as_ptr(), null() as *const c_char) };
        match ret {
            0 => Ok(()),
            _ => Err(current_error().into()),
        }
    }
}

pub enum VipsBandFormat {
    VIPS_FORMAT_NOTSET = -1,
    VIPS_FORMAT_UCHAR = 0,
    VIPS_FORMAT_CHAR = 1,
    VIPS_FORMAT_USHORT = 2,
    VIPS_FORMAT_SHORT = 3,
    VIPS_FORMAT_UINT = 4,
    VIPS_FORMAT_INT = 5,
    VIPS_FORMAT_FLOAT = 6,
    VIPS_FORMAT_COMPLEX = 7,
    VIPS_FORMAT_DOUBLE = 8,
    VIPS_FORMAT_DPCOMPLEX = 9,
    VIPS_FORMAT_LAST = 10,
}

#[cfg(test)]
mod tests {
    use ::*;
    #[test]
    fn it_works() {
        let instance = VipsInstance::new("app_test", true);
        let mut img = VipsImage::new_from_file("kodim01.png").unwrap();
        let mut img = img.thumbnail(123, 234, VipsSize::VIPS_SIZE_FORCE);
        img.write_to_file("kodim01_123x234.png").unwrap();
    }
}
