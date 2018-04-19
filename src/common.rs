// define common enums, etc.

use ::ffi;
use std::ffi::CStr;

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

pub fn current_error() -> String {
    let msg = unsafe {
        CStr::from_ptr(ffi::vips_error_buffer())
    };
    msg.to_str().unwrap().to_string()
}