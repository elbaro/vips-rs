# vips-rs
[![Crates.io](https://img.shields.io/crates/v/vips.svg)](https://crates.io/crates/vips-rs)
[![Build Status](https://travis-ci.org/elbaro/vips-rs.svg?branch=master)](https://travis-ci.org/elbaro/vips-rs)

This crate provides bindings to libvips.

[Documentation](https://elbaro.github.io/vips-rs/vips/)

A binding to `libvips`.

## Usage

```toml
[dependencies]
vips = "*"
```

1. Create a `VipsInstance`.
2. Do your work.
    ```rs
    extern crate vips;
    use vips::*;

    fn main() {
        let instance = VipsInstance::new("app_test", true);
        let img = VipsImage::new_from_file("kodim01.png").unwrap();
        let img = img.thumbnail(123, 234, VipsSize::VIPS_SIZE_FORCE);
        img.write_to_file("kodim01_123x234.png").unwrap();
    }
    ```

## Notes
- The API is incomplete.
- After `VipsInstance` is destroyed, you cannot instantiate another. There is a static boolean variable for checking this.
- If you cannot find an interface you need, you can use `vips-sys` directly, or use `call(op_name, args..)` interface.

## Design To-do
- Easy interface for varargs.
- Add _buf methods to &[u8] as trait?


## How libvips works
- https://jcupitt.github.io/libvips/API/current/How-it-works.md.html

#### Terms
- band: channel
- image: file image / memory (RGB) image / and so on. you cannot directly access a pixel.
- region: sub-area of image. actually read pixels from a image.
- partial image: a function to generate pixels for a rectangular region


#### init/shutdown lifecycle
`libvips` requires the user to call `vips_init()` at the beginning and `vips_shutdown()` at the end.

`vips_shutdown` makes sure async operations finish and all resources are released. Optionally it reports any memory leak.

The binding provides `VipsInstance` for RAII. One peculiar behavior of vips is that after calling `vips_shutdown`, you should not call `vips_init` again. To prevent users from doing this, you can create an instance `VipsInstance` only once in your program's lifetime. When you call `VipsInstance::new` second time (even after the first instance is destroyed), you will get `Result::Err`.

#### Memory Management
`libvips` uses gobject. The memory behind ffi layer is managed by gobject's garbage collection.
If your `VipsImage` owns a rust memory, it registers the deallocation callback to the event when the gobject is destroyed.

```rust
{
    let img2 = {
        // creates new vips::VipsImage that points to the first vips_sys::VipsImage
        let img1 = VipsImage::from_memory(vec);
        // creates new vips::VipsImage that points to the second vips_sys::VipsImage
        img1.some_operation()
    };
    // img1 is destroyed. calls g_object_unref(first vips_sys::VipsImage).
    // the first vips_sys::VipsImage is destroyed
    // because referenced by the cond vips_sys::VipsImage,
}
// img2 is destroyed.
// the second vips_sys::VipsImage is destroyed.
// the first vips_sys::VipsImage is destroyed.
// gobject callbacks the rust deallocator, and vec is destroyed in rust side.
```

#### Owned vs Borrowed
`VipsImage` owns or borrows its pixel data depending on how it is created.

`VipsImage::new_memory`, `VipsImage::from_file` or `VipsImage::from_memory` owns the data.
`VipsImage::from_memory_reference` borrows the pixel data.

If you create a `VipsImage` with a reference to your data, and apply an operation to the image to get another `VipsImage`, your pixel data needs to outlive the second `VipsImage` as well. However, the first `VipsImage` doesn't need to outlive the second one.

You can find about what works and what doesn't in `/tests`.



#### No in-place operation
Vips operations have no side effect on the input image.


#### Memory vs Buffer
You can find these words in API names. For example, there are `vips_image_new_from_memory` and `vips_image_new_from_buffer`. They are not the same.

- memory is a simple (e.g. RGB) array
- buffer is a formatted (jpeg, png, etc) memory data

Some operations directly work on buffer. For example, jpeg buffer can be shrinked during the decoding.
