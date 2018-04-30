#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]
extern crate vips_sys as ffi;
#[macro_use]
extern crate lazy_static;

use std::os::raw::c_void;

// re-exports modules
mod common;
pub use common::*;

mod instance;
pub use instance::VipsInstance;

mod image;
pub use image::VipsImage;

mod buffer;
pub use buffer::VipsBuffer;

// re-exports native enums
pub use ffi::VipsPrecision;
pub use ffi::VipsToken;
pub use ffi::VipsArgumentFlags;
pub use ffi::VipsDemandStyle;
pub use ffi::VipsImageType;
pub use ffi::VipsInterpretation;
pub use ffi::VipsBandFormat;
pub use ffi::VipsCoding;
pub use ffi::VipsAccess;
pub use ffi::VipsFormatFlags;
pub use ffi::VipsOperationFlags;
pub use ffi::VipsForeignFlags;
pub use ffi::VipsSaveable;
pub use ffi::VipsForeignWebpPreset;
pub use ffi::VipsForeignTiffCompression;
pub use ffi::VipsForeignTiffPredictor;
pub use ffi::VipsForeignTiffResunit;
pub use ffi::VipsForeignPngFilter;
pub use ffi::VipsForeignDzLayout;
pub use ffi::VipsForeignDzDepth;
pub use ffi::VipsForeignDzContainer;
pub use ffi::VipsOperationMath;
pub use ffi::VipsOperationMath2;
pub use ffi::VipsOperationRound;
pub use ffi::VipsOperationRelational;
pub use ffi::VipsOperationBoolean;
pub use ffi::VipsOperationComplex;
pub use ffi::VipsOperationComplex2;
pub use ffi::VipsOperationComplexget;
pub use ffi::VipsExtend;
pub use ffi::VipsCompassDirection;
pub use ffi::VipsDirection;
pub use ffi::VipsAlign;
pub use ffi::VipsAngle;
pub use ffi::VipsAngle45;
pub use ffi::VipsInteresting;
pub use ffi::VipsBlendMode;
pub use ffi::VipsCombine;
pub use ffi::VipsOperationMorphology;
pub use ffi::VipsKernel;
pub use ffi::VipsSize;
pub use ffi::VipsIntent;
pub use ffi::VipsPCS;
pub use ffi::VipsCombineMode;
pub use ffi::VipsBBits;

use std::error::Error;


pub fn thumbnail_buffer(data: &[u8], width:u32, height:u32) -> VipsImage {
//        ffi::vips_thumbnail_buffer(data.as_ptr() as *const c_void, data.len(), width, )
    unimplemented!();

}

pub fn jpegload_buffer(buf: &mut [u8]) -> Result<VipsImage, Box<Error>> {
    let mut out = VipsImage::new_memory()?;
    unsafe {
        ffi::vips_jpegload_buffer(buf.as_mut_ptr() as *mut c_void, buf.len(), &mut out.c);
    }
    Ok(out)
}
