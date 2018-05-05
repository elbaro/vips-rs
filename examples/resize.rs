extern crate vips;

use vips::*;

fn resize_file() {
    let thumbnail = {
        let img:VipsImage = VipsImage::from_file("kodim01.png").unwrap();
        img.thumbnail(123, 234, VipsSize::VIPS_SIZE_FORCE).unwrap()
    };

    thumbnail.write_to_file("kodim01_123x234.png").unwrap();
}

fn resize_mem() {
    let pixels = vec![0;256*256*3];
    let thumbnail = {
        let img:VipsImage = VipsImage::from_memory(pixels, 256, 256, 3, VipsBandFormat::VIPS_FORMAT_UCHAR).unwrap();
        img.thumbnail(234, 123, VipsSize::VIPS_SIZE_FORCE).unwrap()
    };
    thumbnail.write_to_file("black_mem_234_123.png").unwrap();
}

fn resize_mem_ref() {
    let pixels = vec![0;256*256*3];
    let thumbnail = {
        let img:VipsImage = VipsImage::from_memory_reference(&pixels, 256, 256, 3, VipsBandFormat::VIPS_FORMAT_UCHAR).unwrap();
        img.thumbnail(234, 123, VipsSize::VIPS_SIZE_FORCE).unwrap()
    };
    thumbnail.write_to_file("black_ref_234x123.png").unwrap();
}

fn main() {
    let _instance = VipsInstance::new("app_test", true).unwrap();
    resize_file();
    resize_mem();
    resize_mem_ref();
}
