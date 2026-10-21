use std::sync::{Arc, Barrier};

#[test]
fn both_crates_share_one_initialization_lock() {
    let barrier = Arc::new(Barrier::new(17));
    let callers: Vec<_> = (0..16)
        .map(|i| {
            let barrier = Arc::clone(&barrier);
            std::thread::spawn(move || {
                barrier.wait();
                if i % 2 == 0 {
                    vips::init(Some("high-level")).unwrap();
                } else {
                    vips_sys::helpers::init("low-level").unwrap();
                }
            })
        })
        .collect();
    barrier.wait();
    for caller in callers {
        caller.join().unwrap();
    }
    assert!(vips::is_initialized());
    assert_eq!(vips::black(2, 2).unwrap().avg().unwrap(), 0.0);
}
