use ::ffi;
use image::VipsImage;

pub struct VipsRegion {
    pub c: *mut ffi::VipsRegion
}

impl VipsRegion {
    pub fn new(image: &VipsImage) -> VipsRegion {
        let c = unsafe {
            ffi::vips_region_new(image.c)
        };
        VipsRegion {
            c
        }
    }
}
