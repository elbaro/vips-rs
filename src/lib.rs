#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(dead_code)]
extern crate vips_sys as ffi;
#[macro_use]
extern crate lazy_static;

// re-exports modules
mod common;
pub use common::*;

mod instance;
pub use instance::VipsInstance;

mod image;
pub use image::VipsImage;

mod interpolate;
pub use interpolate::VipsInterpolate;
pub use interpolate::VipsInterpolateMethod;

mod region;
pub use region::VipsRegion;

mod buffer;
pub use buffer::VipsBuffer;

// re-exports simple structs
pub use ffi::VipsRect;


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


pub use ffi::vips_call as call;

extern "C" {
    pub fn vips_call(operation_name: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
