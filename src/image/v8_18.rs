//! Wrappers for the public API introduced in libvips 8.18.
use super::*;

impl<'a> VipsImage<'a> {
    /// Tile geometry hints supplied by the loader (libvips 8.18+), or `None`
    /// if either dimension is absent or invalid.
    pub fn tile_size(&self) -> Option<(u32, u32)> {
        // SAFETY: these accessors borrow a live image.
        let (width, height) = unsafe {
            (
                vips_sys::vips_image_get_tile_width(self.as_ptr()),
                vips_sys::vips_image_get_tile_height(self.as_ptr()),
            )
        };
        (width > 0 && height > 0).then_some((width as u32, height as u32))
    }

    /// Decode or reference an attached HDR gainmap, if present. Malformed
    /// compressed gainmaps return an error. The returned image owns a reference.
    pub fn gainmap(&self) -> Result<Option<VipsImage<'a>>> {
        // SAFETY: only query metadata on a live image with static field names.
        let present = unsafe {
            vips_sys::vips_image_get_typeof(self.as_ptr(), c"gainmap".as_ptr()) != 0
                || vips_sys::vips_image_get_typeof(self.as_ptr(), c"gainmap-data".as_ptr()) != 0
        };
        if !present {
            return Ok(None);
        }
        // SAFETY: the getter returns a full reference owned by the caller.
        result(unsafe { vips_sys::vips_image_get_gainmap(self.as_ptr()) }).map(Some)
    }

    /// Load a camera RAW file using the optional libraw codec.
    pub fn dcrawload_file(path: impl AsRef<Path>) -> Result<VipsImage<'static>> {
        let path = path_to_cstring(path.as_ref())?;
        let mut out = null_mut();
        // SAFETY: valid path and out pointer; varargs terminated with NULL.
        let rc =
            unsafe { vips_sys::vips_dcrawload(path.as_ptr(), &mut out, null() as *const c_char) };
        result_with_ret(out, rc)
    }

    /// Decode camera RAW bytes using the optional libraw codec.
    pub fn dcrawload_buffer(buf: &'a [u8]) -> Result<VipsImage<'a>> {
        let mut out = null_mut();
        // SAFETY: the result retains the input buffer's lifetime.
        let rc = unsafe {
            vips_sys::vips_dcrawload_buffer(
                buf.as_ptr().cast_mut().cast(),
                buf.len(),
                &mut out,
                null() as *const c_char,
            )
        };
        result_with_ret(out, rc)
    }

    /// Load an UltraHDR JPEG using the optional libultrahdr codec.
    pub fn uhdrload_file(path: impl AsRef<Path>) -> Result<VipsImage<'static>> {
        let path = path_to_cstring(path.as_ref())?;
        let mut out = null_mut();
        // SAFETY: valid path and out pointer; varargs terminated with NULL.
        let rc =
            unsafe { vips_sys::vips_uhdrload(path.as_ptr(), &mut out, null() as *const c_char) };
        result_with_ret(out, rc)
    }

    /// Decode UltraHDR JPEG bytes. The returned lazy image borrows `buf`.
    pub fn uhdrload_buffer(buf: &'a [u8]) -> Result<VipsImage<'a>> {
        let mut out = null_mut();
        // SAFETY: the result retains the input buffer's lifetime.
        let rc = unsafe {
            vips_sys::vips_uhdrload_buffer(
                buf.as_ptr().cast_mut().cast(),
                buf.len(),
                &mut out,
                null() as *const c_char,
            )
        };
        result_with_ret(out, rc)
    }

    /// Save an UltraHDR JPEG. `quality` must be in 1..=100. Requires libultrahdr.
    pub fn write_uhdr(&self, path: impl AsRef<Path>, quality: i32) -> Result<()> {
        validate_quality(quality)?;
        let path = path_to_cstring(path.as_ref())?;
        // SAFETY: image, path and varargs have the C API's required types.
        let rc = unsafe {
            vips_sys::vips_uhdrsave(
                self.as_ptr(),
                path.as_ptr(),
                c"Q".as_ptr(),
                quality,
                null() as *const c_char,
            )
        };
        ffi::ret_to_result(rc)
    }

    /// Encode UltraHDR without copying the encoded bytes into a Rust Vec.
    pub fn write_uhdr_buffer(&self, quality: i32) -> Result<crate::VipsOwnedBuffer> {
        validate_quality(quality)?;
        let mut ptr = null_mut();
        let mut len = 0;
        // SAFETY: valid image and output pointers; success transfers g_free storage.
        let rc = unsafe {
            vips_sys::vips_uhdrsave_buffer(
                self.as_ptr(),
                &mut ptr,
                &mut len,
                c"Q".as_ptr(),
                quality,
                null() as *const c_char,
            )
        };
        let buffer = unsafe { crate::VipsOwnedBuffer::from_raw(ptr, len) }?;
        ffi::ret_to_result(rc)?;
        Ok(buffer)
    }
}

macro_rules! colour_op {
    ($name:ident, $ffi:ident, $doc:literal) => {
        impl<'a> VipsImage<'a> {
            #[doc = $doc]
            pub fn $name(&self) -> Result<VipsImage<'a>> {
                let mut out = null_mut();
                // SAFETY: live image, valid output pointer, NULL-terminated varargs.
                let rc =
                    unsafe { vips_sys::$ffi(self.as_ptr(), &mut out, null() as *const c_char) };
                result_with_ret(out, rc)
            }
        }
    };
}

colour_op!(
    uhdr2scrgb,
    vips_uhdr2scRGB,
    "Apply an UltraHDR gainmap and convert to linear scRGB."
);
colour_op!(
    oklab2xyz,
    vips_Oklab2XYZ,
    "Convert Oklab pixels to XYZ (libvips 8.18+)."
);
colour_op!(
    xyz2oklab,
    vips_XYZ2Oklab,
    "Convert XYZ pixels to Oklab (libvips 8.18+)."
);
colour_op!(
    oklab2oklch,
    vips_Oklab2Oklch,
    "Convert Oklab pixels to Oklch (libvips 8.18+)."
);
colour_op!(
    oklch2oklab,
    vips_Oklch2Oklab,
    "Convert Oklch pixels to Oklab (libvips 8.18+)."
);

fn validate_quality(quality: i32) -> Result<()> {
    if !(1..=100).contains(&quality) {
        return Err(Error::Other("UltraHDR quality must be in 1..=100".into()));
    }
    Ok(())
}
