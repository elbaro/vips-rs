use ffi;
use ffi::{VipsSize, VipsKernel, VipsBandFormat};
use std::error::Error;
use std::os::raw::c_char;
use std::ptr::null;
use std::os::raw::c_void;
use std::ffi::CString;
use common::current_error;
use std::ptr::null_mut;
use std::marker::PhantomData;
use std::os::raw::c_int;

trait VipsImageTrait<'a> {
    type OutputImage: 'a;

    fn ff() -> Result<Self::OutputImage, Box<Error>> {
        Err("abc".into())
    }
}

impl<'a> VipsImageTrait<'a> for VipsImage<'a> {
    type OutputImage = VipsImage<'a>;
}

impl<'a, 'b> VipsImageTrait<'b> for VipsScopedImage {
    type OutputImage = VipsScopedImage;
}

pub struct VipsScopedImage {
//    marker: PhantomData<&'a ()>,
}


pub struct VipsImage<'a> {
    pub c: *mut ffi::VipsImage,
    marker: PhantomData<&'a ()>,
}

impl<'a> Drop for VipsImage<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::g_object_unref(self.c as *mut c_void);
        }
    }
}


fn result<'a>(ptr: *mut ffi::VipsImage) -> Result<VipsImage<'a>, Box<Error>> {
    if ptr.is_null() {
        Err(current_error().into())
    } else {
        Ok(VipsImage { c: ptr, marker: PhantomData })
    }
}

fn result_with_ret<'a>(ptr: *mut ffi::VipsImage, ret: c_int) -> Result<VipsImage<'a>, Box<Error>> {
    if ret == 0 {
        Ok(VipsImage { c: ptr, marker: PhantomData })
    } else {
        Err(current_error().into())
    }
}


impl<'a> VipsImage<'a> {
    pub fn new() -> Result<VipsImage<'a>, Box<Error>> {
        let c = unsafe { ffi::vips_image_new() };
        result(c)
    }

    pub fn new_memory() -> Result<VipsImage<'a>, Box<Error>> {
        let c = unsafe { ffi::vips_image_new_memory() };
        result(c)
    }

    pub fn from_file<S: Into<Vec<u8>>>(path: S) -> Result<VipsImage<'a>, Box<Error>> {
        let path = CString::new(path)?;

        let c = unsafe { ffi::vips_image_new_from_file(path.as_ptr(), null() as *const c_char) };
        result(c)
    }

    pub fn from_memory(buf: &'a [u8], width: u32, height: u32, bands: u8, format: VipsBandFormat) -> Result<VipsImage<'a>, Box<Error>> {
        let c = unsafe {
            ffi::vips_image_new_from_memory(
                buf.as_ptr() as *const c_void,
                buf.len(),
                width as i32,
                height as i32,
                bands as i32,
                format,
            )
        };

        result(c)
    }

    // formatted
    pub fn from_buffer(buf: &'a [u8]) -> Result<VipsImage<'a>, Box<Error>> {
        let c = unsafe {
            ffi::vips_image_new_from_buffer(buf.as_ptr() as *const c_void, buf.len(), null(), null() as *const c_char)
        };

        result(c)
    }

    fn width(&self) -> u32 {
        unsafe { (*self.c).Xsize as u32 }
    }

    fn height(&self) -> u32 {
        unsafe { (*self.c).Ysize as u32 }
    }

    pub fn thumbnail(&self, width: u32, height: u32, size: VipsSize) -> Result<VipsImage<'a>, Box<Error>> {
        let mut out_ptr: *mut ffi::VipsImage = null_mut();
        unsafe {
            ffi::vips_thumbnail_image(self.c as *mut ffi::VipsImage, &mut out_ptr, width as i32, "height\0".as_ptr(), height as i32, "size\0".as_ptr(), size, null() as *const c_char);
        };
        result(out_ptr)
    }

    // default: block shrink + lanczos3
    fn resize(&self, scale: f64, vscale: Option<f64>, kernel: Option<VipsKernel>) -> Result<VipsImage<'a>, Box<Error>> {
        let mut out_ptr: *mut ffi::VipsImage = null_mut();
        let ret = unsafe {
            ffi::vips_resize(
                self.c as *mut ffi::VipsImage,
                &mut out_ptr,
                scale,
                "vscale\0".as_ptr(),
                vscale.unwrap_or(scale),
                "kernel\0".as_ptr(),
                kernel.unwrap_or(VipsKernel::VIPS_KERNEL_LANCZOS3),
                null() as *const c_char,
            )
        };
        result_with_ret(out_ptr, ret)
    }
    fn resize_to_size(&self, width: u32, height: Option<u32>, kernel: Option<VipsKernel>) -> Result<VipsImage<'a>, Box<Error>> {
        self.resize(
            width as f64 / self.width() as f64,
            height.map(|h| h as f64 / self.height() as f64),
            kernel,
        )
    }

    // low-level
    // default: 2 * 1D lanczos3 (not recommended for shrink factor > 3)
    // or other kernels
    fn reduce(&self, hshrink: f64, vshrink: f64, kernel: Option<VipsKernel>, centre: Option<bool>) -> VipsImage {
        unimplemented!();
//        unsafe {
//            ffi::vips_reduce(self.c, , )
//        }
    }

    fn shrink(&self) -> VipsImage { // simple average of nxn -> 1/n size
        unimplemented!();
    }

    fn jpegsave<S: Into<Vec<u8>>>(&mut self, path: S) -> Result<(), Box<Error>> {
        let path = CString::new(path)?;
        let ret = unsafe { ffi::vips_jpegsave(self.c as *mut ffi::VipsImage, path.as_ptr(), null() as *const c_char) };
        match ret {
            0 => Ok(()),
            _ => Err(current_error().into()),
        }
    }

    pub fn write_to_file<S: Into<Vec<u8>>>(&self, path: S) -> Result<(), Box<Error>> {
        let path = CString::new(path)?;
        let ret = unsafe { ffi::vips_image_write_to_file(self.c as *mut ffi::VipsImage, path.as_ptr(), null() as *const c_char) };
        match ret {
            0 => Ok(()),
            _ => Err(current_error().into()),
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        unsafe {
            let mut result_size: usize = 0;
            let memory: *mut u8 = ffi::vips_image_write_to_memory(self.c as *mut ffi::VipsImage, &mut result_size as *mut usize) as *mut u8;
            let slice = ::std::slice::from_raw_parts_mut(memory, result_size);
            let boxed_slice: Box<[u8]> = Box::from_raw(slice);
            let vec = boxed_slice.into_vec();
            vec
        }
    }
}
