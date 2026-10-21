use vips::*;

fn main() {
    let base = black(2, 2).unwrap();
    let output = {
        let pixels = vec![42; 4];
        let other = VipsImage::from_memory_reference(
            &pixels, 2, 2, 1, VipsBandFormat::VIPS_FORMAT_UCHAR,
        ).unwrap();
        base.add(&other).unwrap()
    };
    let _ = output.write_to_memory();
}
