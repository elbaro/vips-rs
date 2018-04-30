# vips-rs
[![Crates.io](https://img.shields.io/crates/v/vips-rs.svg)](https://crates.io/crates/vips-rs)
[![Build Status](https://travis-ci.org/elbaro/vips-rs.svg?branch=master)](https://travis-ci.org/elbaro/vips-rs)

This crate provides bindings to libvips.

[Documentation](https://elbaro.github.io/vips-rs/vips_rs/)

A binding to `libvips`.

## Notes

- This crate is rename from "vips-rs" to "vips".
- The API is unstable.
- If you cannot find an interface you need, you can use `vips-sys` directly.


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
- Should `VipsImage` enforce ownership?
- Easy interface for varargs.
- Add _buf methods to &[u8] as trait?


## How Vips works
- https://jcupitt.github.io/libvips/API/current/How-it-works.md.html

#### Terms
- band: channel
- image: image file / memory (RGB) / buffer (jpeg, png) / ..
- region: sub-area of image. actually read pixels from a image.
- partial image: a function to generate pixels for a rectangular region


#### init/shutdown lifecycle
`libvips` requires the user to call `vips_init()` at the beginning and `vips_shutdown()` at the end.

`vips_shutdown` makes sure async operations finish and all resources are released. Optionally it reports any memory leak.

The binding provides `VipsInstance` for RAII. One peculiar behavior of vips is that after calling `vips_shutdown`, you should not call `vips_init` again. To prevent users from doing this, you can create an instance `VipsInstance` only once in your program's lifetime. When you call `VipsInstance::new` second time (even after the first instance is destroyed), you will get `Result::Err`.

#### Memory Management
Everything in `libvips` is gobject. The binding classes call  g_object_unref` on Drop.

#### No in-place operation
Vips operations have no side effect. However it may share the data between images to avoid copying.

```
let img = {
    let large_img1 = VipsImage::new..(..);
    let large_img2 = small_change(&large_img1); // fast
    large_img2
}
// large_img1 is dropped,
// but its pixel data is being used by large_img2
```

```
{
    let data = ..;
    let img1 = VipsImage::new_from_reference(&data);
    let img2 = img1.resize(..);

    // data needs to outlive img1
    // img1 needs to outlive img2
}
```
The input image of any operation needs to outlive the output image.


#### Memory vs Buffer
You can find these words in API names. For example, there are `vips_image_new_from_memory` and `vips_image_new_from_buffer`. They are not the same.

- memory is a simple (e.g. RGB) array
- buffer is a formatted (jpeg, png, etc) memory data

Some operations directly work on buffer. For example, jpeg buffer can be shrinked during the decoding.

#### VipsImage: from owned memory

Creating a new VipsImage from owned data is easy and safe.
```
{
    let vec:Vec<u8> = ..;
    // transfer vec to VipsImage
    // VipsImage transfers pixel data to
    let img = VipsImage::from_memory(vec, width, height, ..);
}
// img is destroyed. pixel data may not be released.
```

Creating a new VipsImage from reference is tricky.
```
{
    let vec:Vec<u8> = ..;
    let img = VipsImage;:from_memory_reference(&vec[..], width, height, ..);
}
// img is destroyed.
```

`VipsImage` in `libvips` is not a pixel tensor like `cv::Mat` or `ndarray`.
It may represent an image file, owns a pixel data in memory, or

#### VipsImage: from memory



#### VipsImage: from buffer

