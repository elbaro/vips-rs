use vips::*;

fn main() {
    let region = {
        let pixels = vec![42; 4];
        let image = VipsImage::from_memory_reference(
            &pixels, 2, 2, 1, VipsBandFormat::VIPS_FORMAT_UCHAR,
        ).unwrap();
        VipsRegion::new(&image).unwrap()
    };
    let _ = region.as_ptr();
}
