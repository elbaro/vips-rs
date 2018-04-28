use ffi;
use ffi::{VipsSize, VipsKernel, VipsBandFormat};
use std::error::Error;
use std::os::raw::c_char;
use std::ptr::null;
use std::os::raw::c_void;
use std::ffi::CString;
use common::current_error;
use std::ptr::null_mut;


pub struct VipsImage<'a> {
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

impl VipsImage<'a> {
    pub fn new() -> Result<VipsImage, Box<Error>> {
        unsafe { Ok(VipsImage { c: ffi::vips_image_new() }) }
//        let c = unsafe { ffi::vips_image_new() }.as_ref();
//        match c {
//            Some(c) => Ok(VipsImage { c }),
//            None => Err(current_error().into())
//        }
    }

    pub fn new_memory() -> Result<VipsImage, Box<Error>> {
        unsafe { Ok(VipsImage { c: ffi::vips_image_new_memory() }) }
//        let c = unsafe { ffi::vips_image_new_memory() }.as_ref();
//        match c {
//            Some(c) => Ok(VipsImage { c }),
//            None => Err(current_error().into())
//        }
    }


    pub fn from_file<S: Into<Vec<u8>>>(path: S) -> Result<VipsImage, Box<Error>> {
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

    pub fn from_memory() {

    }
}

pub trait VipsImageTrait {
    fn c_data(&self) -> *const ffi::VipsImage;
    fn c_data_mut(&mut self) -> *mut ffi::VipsImage;

    fn width(&self) -> u32 {
        unsafe { (*self.c_data()).Xsize as u32 }
    }

    fn height(&self) -> u32 {
        unsafe { (*self.c_data()).Ysize as u32 }
    }

    fn thumbnail(&self, width:u32, height:u32, size:VipsSize) -> Result<VipsImage, Box<Error>> {
        let mut out = VipsImage::new_memory()?;
        unsafe {
            ffi::vips_thumbnail_image(self.c_data() as *mut ffi::VipsImage, &mut out.c, width as i32, "height\0".as_ptr(), height as i32, "size\0".as_ptr(), size, null() as *const c_char);
        }
        Ok(out)
    }

    // default: block shrink + lanczos3
    fn resize(&self, out: Option<VipsImage>, scale:f64, vscale:Option<f64>, kernel:Option<VipsKernel>) -> Result<VipsImage, Box<Error>> {
        let mut out = out.unwrap_or(VipsImage::new_memory()?);
        unsafe {
            ffi::vips_resize(
                self.c_data() as *mut ffi::VipsImage,
                &mut out.c,
                scale,
                "vscale\0".as_ptr(),
                vscale.unwrap_or(scale),
                "kernel\0".as_ptr(),
                kernel.unwrap_or(VipsKernel::VIPS_KERNEL_LANCZOS3),
                null() as *const c_char
            );
        }
        Ok(out)
    }
    fn resize_to_size(&self, out: Option<VipsImage>, width:u32, height:Option<u32>, kernel:Option<VipsKernel>) -> Result<VipsImage, Box<Error>> {
        self.resize(
            out,
            width as f64/self.width() as f64,
            height.map(|h| h as f64/self.height() as f64),
            kernel
        )
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
        let ret = unsafe { ffi::vips_jpegsave(self.c_data() as *mut ffi::VipsImage, path.as_ptr(), null() as *const c_char) };
        match ret {
            0 => Ok(()),
            _ => Err(current_error().into()),
        }
    }

    fn write_to_file<S:Into<Vec<u8>>>(&mut self, path: S) -> Result<(),Box<Error>> {
        let path = CString::new(path)?;
        let ret = unsafe { ffi::vips_image_write_to_file(self.c_data() as *mut ffi::VipsImage, path.as_ptr(), null() as *const c_char) };
        match ret {
            0 => Ok(()),
            _ => Err(current_error().into()),
        }
    }

    fn to_vec(&self) -> Vec<u8> {
        unsafe {
            let mut result_size:usize = 0;
            let memory: *mut u8 = ffi::vips_image_write_to_memory(self.c_data() as *mut ffi::VipsImage, &mut result_size as *mut usize) as *mut u8;
            let slice = ::std::slice::from_raw_parts_mut(memory, result_size);
            let boxed_slice:Box<[u8]> = Box::from_raw(slice);
            let vec = boxed_slice.into_vec();
            vec
        }
    }
}

impl<'a> VipsImageTrait for VipsImage<'a> {
    fn c_data(&self) -> *const ffi::VipsImage {
        self.c
    }
    fn c_data_mut(&mut self) -> *mut ffi::VipsImage {
        self.c
    }
}

impl<'a> VipsImageTrait for VipsBufferImage<'a> {
    fn c_data(&self) -> *const ffi::VipsImage {
        self.c
    }
    fn c_data_mut(&mut self) -> *mut ffi::VipsImage {
        self.c
    }
}

impl<'a> VipsBufferImage<'a>{
    // unformatted (e.g. RGB vs formatted JPEG)
    pub fn new_from_memory(buf: &'a [u8], width: u32, height: u32, bytes_per_pixel: u32, format: VipsBandFormat) -> Result<VipsBufferImage<'a>, Box<Error>> {
        let img = unsafe {
            ffi::vips_image_new_from_memory(buf.as_ptr() as *const c_void, buf.len(), width as i32, height as i32, bytes_per_pixel as i32, format)
        };


        if img!=null_mut() {
            Ok(VipsBufferImage {
                buf,
                c: img
            })
        } else {
            Err(current_error().into())
        }
    }

    // formatted
    pub fn new_from_buffer(buf: &'a [u8]) -> Result<VipsBufferImage<'a>, Box<Error>> {
        let img = unsafe {
            ffi::vips_image_new_from_buffer(buf.as_ptr() as *const c_void, buf.len(), null(), null() as *const c_char)
        };


        if img!=null_mut() {
            Ok(VipsBufferImage {
                buf,
                c: img
            })
        } else {
            Err(current_error().into())
        }
    }
}
