use vips::*;

fn main() {
    let image = black(2, 2).unwrap();
    let region = VipsRegion::new(&image).unwrap();
    let interpolate = VipsInterpolate::bilinear_static();
    interpolate.method().call(&interpolate, &region, &mut [], 0.0, 0.0);
}
