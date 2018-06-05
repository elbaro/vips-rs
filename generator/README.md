```sh
~/vips-rs/generator master*
❯ python method.py < method_example.txt
pub fn mosaic1(&self, ref : VipsImage, sec : VipsImage, direction : VipsDirection, xr1 : int, yr1 : int, xs1 : int, ys1 : int, xr2 : int, yr2 : int, xs2 : int, ys2 : int, search : Option<>, hwindow : Option<>, harea : Option<>, interpolate : Option<>, mblend : Option<>, bandno : Option<>) -> Result<VipsImage, Box<Error>> {
    let mut out_ptr: *mut ffi::VipsImage = null_mut();
    let ret = unsafe {
        ffi::vips_merge(
            self.c as *mut ffi::VipsImage,
            ref.c as *mut ffi::VipsImage,
            sec.c as *mut ffi::VipsImage,
            &mut out_ptr,
            direction,
            xr1,
            yr1,
            xs1,
            ys1,
            xr2,
            yr2,
            xs2,
            ys2,
            search.unwrap_or(),
            hwindow.unwrap_or(),
            harea.unwrap_or(),
            interpolate.unwrap_or(),
            mblend.unwrap_or(),
            bandno.unwrap_or(),
            null() as *const c_char);
    };
    result_with_ret(out_ptr, ret)
}
```