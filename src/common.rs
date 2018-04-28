// define common enums, etc.

use ::ffi;
use std::ffi::CStr;


pub fn current_error() -> String {
    let msg = unsafe {
        CStr::from_ptr(ffi::vips_error_buffer())
    };
    msg.to_str().unwrap().to_string()
}