use vips::*;

fn main() -> Result<()> {
    init(Some("secondary-owned"))?;
    let base = black(2, 2)?;
    let pixels = vec![42; 4];
    let output = {
        let other = VipsImage::from_memory_reference(
            &pixels, 2, 2, 1, VipsBandFormat::VIPS_FORMAT_UCHAR,
        )?;
        base.add(&other)?
    };
    assert_eq!(output.avg()?, 42.0);
    Ok(())
}
