//! Run with `cargo run --release --example buffer_bench`.
//! Measures actual pixel export and destruction, with alternating ABBA samples.
use std::hint::black_box;
use std::time::{Duration, Instant};
use vips::*;

fn sample(image: &VipsImage<'_>, owned: bool) -> Result<Duration> {
    let start = Instant::now();
    if owned {
        let output = image.write_to_memory_owned()?;
        black_box(output.as_slice());
    } else {
        let output = image.write_to_memory()?;
        black_box(output.as_slice());
    }
    Ok(start.elapsed())
}

fn median(samples: &mut [Duration]) -> Duration {
    samples.sort_unstable();
    samples[samples.len() / 2]
}

fn main() -> Result<()> {
    init(Some("buffer-bench"))?;
    let image = VipsImage::from_memory(
        vec![127; 2048 * 2048 * 3],
        2048,
        2048,
        3,
        VipsBandFormat::VIPS_FORMAT_UCHAR,
    )?;
    for _ in 0..3 {
        sample(&image, false)?;
        sample(&image, true)?;
    }
    let mut copied = Vec::new();
    let mut owned = Vec::new();
    let mut first_baseline = Vec::new();
    let mut last_baseline = Vec::new();
    for _ in 0..10 {
        for (phase, mode) in [false, true, true, false].into_iter().enumerate() {
            let elapsed = sample(&image, mode)?;
            if mode {
                owned.push(elapsed);
            } else {
                copied.push(elapsed);
            }
            if phase == 0 {
                first_baseline.push(elapsed);
            }
            if phase == 3 {
                last_baseline.push(elapsed);
            }
        }
    }
    let copied = median(&mut copied);
    let owned = median(&mut owned);
    let first = median(&mut first_baseline).as_secs_f64();
    let last = median(&mut last_baseline).as_secs_f64();
    let drift = (last / first - 1.0).abs();
    println!(
        "libvips={} pixels=2048x2048x3 bytes={} samples_per_mode=20 copied_median={copied:?} owned_median={owned:?} baseline_drift={drift:.3}",
        version_string(),
        2048 * 2048 * 3
    );
    if drift <= 0.10 {
        println!(
            "ratio={:.3} baseline_drift_gate=pass",
            owned.as_secs_f64() / copied.as_secs_f64()
        );
    } else {
        println!("baseline_drift_gate=failed: inconclusive; rerun under a stable workload");
    }
    Ok(())
}
