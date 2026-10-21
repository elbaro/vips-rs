use vips::*;

#[test]
fn initialization_retries_and_instance_drop_keeps_live_images_usable() -> Result<()> {
    assert!(VipsInstance::new("bad\0name", false).is_err());
    assert!(!is_initialized());
    let image = {
        let _instance = VipsInstance::new("initialization-test", false)?;
        assert!(is_initialized());
        black(2, 2)?
    };
    assert_eq!(image.avg()?, 0.0);
    assert!(VipsInstance::new("duplicate", false).is_err());
    let callers: Vec<_> = (0..8).map(|_| std::thread::spawn(|| init(None))).collect();
    for caller in callers {
        caller.join().unwrap()?;
    }
    Ok(())
}
