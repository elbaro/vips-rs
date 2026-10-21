//! Process-wide initialization. Global resources remain available for the
//! process lifetime; Rust statics do not run destructors at process exit.

use crate::{Error, Result, take_vips_error};

/// Initialize libvips (idempotent, thread-safe). `app_name` is used for logging
/// and diagnostics; defaults to `"vips-rs"` when `None`.
///
/// Concurrent first-time callers take a process-wide lock so only one thread
/// performs `vips_init`.
///
/// # Errors
/// Returns `Error::InitFailed` if initialization fails.
///
/// # Example
/// ```no_run
/// use vips::*;
///
/// fn main() -> Result<()> {
///     init(Some("my_app"))?;
///     // Your libvips code here
///     Ok(())
/// }
/// ```
pub fn init(app_name: Option<&str>) -> Result<()> {
    if vips_sys::helpers::is_initialized() {
        return Ok(());
    }

    let name = app_name.unwrap_or("vips-rs");
    if name.contains('\0') {
        return Err(Error::InitFailed("invalid app name: contains NUL".into()));
    }
    // Share the low-level initialization lock with direct users of helpers.
    if vips_sys::helpers::init(name).is_err() {
        let msg = take_vips_error().unwrap_or_else(|| "vips_init() failed".to_string());
        return Err(Error::InitFailed(msg));
    }

    Ok(())
}

/// Is it currently initialized?
///
/// # Returns
/// `true` if libvips has been initialized, `false` otherwise.
///
/// # Example
/// ```no_run
/// use vips::is_initialized;
///
///  if is_initialized() {
///     println!("libvips is initialized");
///  } else {
///     println!("libvips is not initialized");
///  }
/// ```
#[inline]
pub fn is_initialized() -> bool {
    vips_sys::helpers::is_initialized()
}
