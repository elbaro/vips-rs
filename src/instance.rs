use crate::{Error, Result};
use std::os::raw::c_int;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

static IS_INSTANTIATED: AtomicBool = AtomicBool::new(false);

/// A singleton compatibility handle for process-wide libvips initialization.
/// Dropping it leaves libvips available to live images and other threads.
/// # Example
/// ```no_run
/// use vips::*;
///
/// fn main() -> Result<()> {
///     let _instance = VipsInstance::new("app_test", true)?;
///     // Your libvips code here
///     Ok(())
/// }
/// ```
pub struct VipsInstance {}

impl VipsInstance {
    /// Create a new VipsInstance, initializing libvips.
    /// This can only be done once per program execution.
    /// Subsequent attempts will return an error.
    /// # Arguments
    /// * `name` - Application name for libvips initialization.
    /// * `leak_test` - If true, enables leak testing in libvips.
    ///
    /// # Errors
    /// Returns an error if an instance already exists.
    ///
    /// # Example
    /// ```no_run
    /// use vips::*;
    ///
    /// fn main() -> Result<()> {
    ///     let _instance = VipsInstance::new("app_test", true)?;
    ///     // Your libvips code here
    ///     Ok(())
    /// }
    /// ```
    /// Uses the same initialization lock as [`crate::init`]. Initialization
    /// failures are returned and do not consume the singleton slot.
    pub fn new(name: &str, leak_test: bool) -> Result<VipsInstance> {
        crate::init(Some(name))?;
        // Try to set false -> true, allowing only once
        match IS_INSTANTIATED.compare_exchange(false, true, Relaxed, Relaxed) {
            Ok(_) => {
                unsafe {
                    if leak_test {
                        vips_sys::vips_leak_set(leak_test as c_int);
                    }
                }
                Ok(VipsInstance {})
            }
            Err(_) => Err(Error::Other(
                "You cannot create VipsInstance more than once.".to_string(),
            )),
        }
    }
}
