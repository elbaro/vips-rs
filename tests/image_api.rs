use vips::*;

fn memory_image(pixels: Vec<u8>) -> Result<VipsImage<'static>> {
    VipsImage::from_memory(pixels, 2, 2, 1, VipsBandFormat::VIPS_FORMAT_UCHAR)
}

#[test]
fn output_buffer_owns_pixels_after_the_image_is_dropped() -> Result<()> {
    init(None)?;
    let buffer = memory_image(vec![1, 2, 3, 4])?.write_to_memory_owned()?;
    assert_eq!(buffer.as_ref(), &[1, 2, 3, 4]);
    let moved = std::thread::spawn(move || buffer.as_slice().to_vec());
    assert_eq!(moved.join().unwrap(), vec![1, 2, 3, 4]);
    Ok(())
}

#[test]
fn owned_input_retains_spare_capacity_without_reallocation() -> Result<()> {
    init(None)?;
    let mut pixels = Vec::with_capacity(1024);
    pixels.extend_from_slice(&[1, 2, 3, 4]);
    let original = pixels.as_ptr();
    let image = memory_image(pixels)?;
    // SAFETY: image is private to this thread and owns the pixel allocation.
    assert_eq!(
        unsafe { vips_sys::vips_image_get_data(image.as_ptr()) }.cast::<u8>(),
        original
    );
    Ok(())
}

#[test]
fn drawing_detaches_an_immutable_borrowed_buffer() -> Result<()> {
    init(None)?;
    let pixels = [1, 2, 3, 4];
    let mut image =
        VipsImage::from_memory_reference(&pixels, 2, 2, 1, VipsBandFormat::VIPS_FORMAT_UCHAR)?;
    image.draw_point1(99.0, 0, 0)?;
    image.draw_point1(88.0, 1, 1)?;
    assert_eq!(pixels, [1, 2, 3, 4]);
    assert_eq!(image.write_to_memory()?, [99, 2, 3, 88]);
    Ok(())
}

#[test]
fn region_retains_the_image_reference() -> Result<()> {
    init(None)?;
    let region = {
        let image = memory_image(vec![1, 2, 3, 4])?;
        VipsRegion::new(&image).unwrap()
    };
    let rect = VipsRect {
        left: 0,
        top: 0,
        width: 2,
        height: 2,
    };
    // SAFETY: the requested rectangle lies within the retained image.
    assert_eq!(
        unsafe { vips_sys::vips_region_prepare(region.as_ptr(), &rect) },
        0
    );
    // SAFETY: region is live and owns an image reference.
    assert_eq!(unsafe { vips_sys::vips_region_width(region.as_ptr()) }, 2);
    Ok(())
}

#[test]
fn memory_shape_and_alignment_are_validated() -> Result<()> {
    init(None)?;
    assert!(memory_image(vec![1]).is_err());
    assert!(VipsImage::from_memory(vec![], 0, 1, 1, VipsBandFormat::VIPS_FORMAT_UCHAR).is_err());
    assert!(
        VipsImage::from_memory(
            vec![0; 4],
            u32::MAX,
            1,
            1,
            VipsBandFormat::VIPS_FORMAT_UCHAR
        )
        .is_err()
    );
    let aligned = [0u16; 5];
    // SAFETY: every byte of the initialized u16 array is readable.
    let bytes = unsafe { std::slice::from_raw_parts(aligned.as_ptr().cast::<u8>(), 10) };
    assert!(
        VipsImage::from_memory_reference(&bytes[1..], 2, 2, 1, VipsBandFormat::VIPS_FORMAT_USHORT)
            .is_err()
    );
    Ok(())
}

#[test]
fn invalid_filter_parameters_return_errors_without_panicking() -> Result<()> {
    init(None)?;
    let image = black(2, 2)?;
    assert!(image.linear(&[1.0], &[]).is_err());
    assert!(image.linear(&[], &[]).is_err());
    assert!(image.median(i32::MAX).is_err());
    assert!(image.median(0).is_err());
    Ok(())
}

#[test]
fn encoded_owned_output_and_sequential_load_round_trip() -> Result<()> {
    init(None)?;
    let encoded = memory_image(vec![1, 2, 3, 4])?.write_to_buffer_owned(".png")?;
    let image = VipsImage::from_buffer_with_access(&encoded, VipsAccess::VIPS_ACCESS_SEQUENTIAL)?;
    assert_eq!(image.write_to_memory()?, [1, 2, 3, 4]);
    assert!(
        black(1, 1)?
            .write_to_buffer_owned(".no-such-format")
            .is_err()
    );
    Ok(())
}

#[cfg(feature = "v8_18")]
#[test]
fn stable_8_18_metadata_and_colour_operations() -> Result<()> {
    init(None)?;
    assert_eq!(
        interpretation_bands(VipsInterpretation::VIPS_INTERPRETATION_sRGB),
        3
    );
    let image = black(4, 4)?
        .bandjoin_const(&[0.0, 0.0])?
        .linear1(1.0, 20.0)?;
    assert_eq!(image.tile_size(), None);
    let tiled = memory_image(vec![1, 2, 3, 4])?;
    // SAFETY: metadata is set only on this thread's private image header.
    unsafe {
        vips_sys::vips_image_set_int(tiled.as_ptr(), c"tile-width".as_ptr(), 2);
        vips_sys::vips_image_set_int(tiled.as_ptr(), c"tile-height".as_ptr(), 2);
    }
    assert_eq!(tiled.tile_size(), Some((2, 2)));
    assert!(image.gainmap()?.is_none());
    let roundtrip = image
        .xyz2oklab()?
        .oklab2oklch()?
        .oklch2oklab()?
        .oklab2xyz()?;
    assert!(roundtrip.subtract(&image)?.abs()?.max_value()? < 0.01);
    assert!(image.write_uhdr_buffer(101).is_err());
    assert!(VipsImage::dcrawload_buffer(b"invalid raw").is_err());
    assert!(VipsImage::uhdrload_buffer(b"invalid hdr").is_err());
    Ok(())
}

#[cfg(feature = "v8_18")]
#[test]
fn ultrahdr_round_trip_or_explicit_unavailable_codec_error() -> Result<()> {
    init(None)?;
    let source = VipsImage::from_file("examples/images/kodim01.png")?
        .colourspace(VipsInterpretation::VIPS_INTERPRETATION_scRGB)?;
    // SAFETY: vips_type_find accepts static strings and only queries registration.
    let available =
        unsafe { vips_sys::vips_type_find(c"VipsOperation".as_ptr(), c"uhdrsave_buffer".as_ptr()) }
            != 0;
    if !available {
        assert!(source.write_uhdr_buffer(80).is_err());
        eprintln!("libultrahdr unavailable: verified unsupported-codec error path");
        return Ok(());
    }
    let encoded = source.write_uhdr_buffer(80)?;
    let loaded = VipsImage::uhdrload_buffer(&encoded)?;
    assert_eq!(loaded.size(), source.size());
    assert!(loaded.gainmap()?.is_some());
    assert_eq!(loaded.uhdr2scrgb()?.size(), source.size());
    let file = std::env::temp_dir().join(format!("vips-uhdr-{}.jpg", std::process::id()));
    struct Cleanup(std::path::PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }
    let _cleanup = Cleanup(file.clone());
    source.write_uhdr(&file, 80)?;
    let loaded = VipsImage::uhdrload_file(&file)?;
    assert_eq!(loaded.uhdr2scrgb()?.size(), source.size());
    Ok(())
}
