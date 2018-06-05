use ffi;
use std::error::Error;
use std::ffi::CString;
use common::current_error;
use std::os::raw::c_void;


pub struct VipsInterpolate {
    pub c: *mut ffi::VipsInterpolate,
    is_static: bool,
}

impl<'a> Drop for VipsInterpolate {
    fn drop(&mut self) {
        if !self.is_static {
            unsafe {
                ffi::g_object_unref(self.c as *mut c_void);
            }
        }
    }
}


impl VipsInterpolate {

    //
    // ─── CONSTRUCTORS ───────────────────────────────────────────────────────────────
    //

    pub fn new(nickname: &str) -> Result<VipsInterpolate, Box<Error>> {
        let nickname = CString::new(nickname)?;
        let c = unsafe { ffi::vips_interpolate_new(nickname.as_ptr()) };
        if c.is_null() {
            Err(current_error().into())
        } else {
            Ok(VipsInterpolate { c, is_static:false })
        }
    }

    pub fn nearest_static() -> VipsInterpolate {
        let c = unsafe { ffi::vips_interpolate_nearest_static() };
        VipsInterpolate {c, is_static:true}
    }

    pub fn bilinear_static() -> VipsInterpolate {
        let c = unsafe { ffi::vips_interpolate_bilinear_static() };
        VipsInterpolate {c, is_static:true}
    }
}
