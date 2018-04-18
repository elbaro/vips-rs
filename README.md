# vips-rs

A binding to `libvips`.

## Notes

- The API is unstable.
- Only a portion of `libvips` is implemented.
If you cannot find an interface you need, you can use `vips-sys` directly.

## Example

```rs
extern crate vips_rs;
use vips_rs::*;

fn main() {
    let instance = VipsInstance::new("app_test", true);
    let mut img = VipsImage::new_from_file("kodim01.png").unwrap();
    let mut img = img.thumbnail(123, 234, VipsSize::VIPS_SIZE_FORCE);
    img.write_to_file("kodim01_123x234.png").unwrap();
}
```

## Design To-do
- How to prevent users from calling `vips_shutdown` after `vips_init`?
- Should `VipsImage` enforce ownership?
- Easy interface for varargs.
- Add _buf methods to &[u8] as trait?
