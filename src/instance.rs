use std::sync::atomic::AtomicBool;
use std::ffi::CString;
use std::error::Error;
use std::os::raw::c_int;
use std::sync::atomic::Ordering::Relaxed;
use ::ffi;

lazy_static! {
    static ref IS_INSTANCIATED: AtomicBool = AtomicBool::new(false);
}

pub struct VipsInstance { }

impl VipsInstance {
    pub fn new(name:&str, leak_test:bool) -> Result<VipsInstance, Box<Error>> {
        // cas return value: prev value
        if IS_INSTANCIATED.compare_and_swap(false, true, Relaxed) {
            Err("You cannot create VipsInstance more than once.".into())
        } else {
            let c = CString::new(name)?;
            unsafe {
                ffi::vips_init(c.as_ptr());
                if leak_test {
                    ffi::vips_leak_set(leak_test as c_int);
                }
            }
            Ok(VipsInstance {})
        }
    }
}

impl Drop for VipsInstance {
    fn drop(&mut self) {
        unsafe {
            ffi::vips_shutdown();
        }
    }
}