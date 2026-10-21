use crate::{Error, Result};
use std::ffi::c_void;
use std::ops::Deref;
use std::ptr::NonNull;

/// An immutable libvips allocation. Reading it does not copy the pixel or
/// encoded data into the Rust allocator. Drop releases it with `g_free`.
#[derive(Debug)]
pub struct VipsOwnedBuffer {
    ptr: Option<NonNull<u8>>,
    len: usize,
}

// SAFETY: the allocation is exclusively owned, exposed only as immutable
// bytes, and GLib permits g_free from any thread.
unsafe impl Send for VipsOwnedBuffer {}
unsafe impl Sync for VipsOwnedBuffer {}

impl VipsOwnedBuffer {
    /// # Safety
    /// `ptr` transfers a g_free-compatible allocation of `len` initialized
    /// bytes, or is null with zero length. No other owner may free it.
    pub(crate) unsafe fn from_raw(ptr: *mut c_void, len: usize) -> Result<Self> {
        let buffer = Self {
            ptr: NonNull::new(ptr.cast()),
            len,
        };
        if len > isize::MAX as usize || (len != 0 && buffer.ptr.is_none()) {
            return Err(Error::Other(
                "invalid libvips output buffer length or pointer".into(),
            ));
        }
        Ok(buffer)
    }

    /// The output bytes, valid for the lifetime of this allocation.
    pub fn as_slice(&self) -> &[u8] {
        if self.len == 0 {
            return &[];
        }
        // SAFETY: from_raw validated length and non-nullness; ownership and
        // immutability keep this allocation valid for the borrow.
        unsafe { std::slice::from_raw_parts(self.ptr.unwrap().as_ptr(), self.len) }
    }
}

impl Deref for VipsOwnedBuffer {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl AsRef<[u8]> for VipsOwnedBuffer {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Drop for VipsOwnedBuffer {
    fn drop(&mut self) {
        if let Some(ptr) = self.ptr {
            // SAFETY: this type holds the sole owner of this allocation.
            unsafe { vips_sys::g_free(ptr.as_ptr().cast()) };
        }
    }
}
