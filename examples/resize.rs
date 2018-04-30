extern crate vips;

use vips::*;

fn main() {
    let _instance = VipsInstance::new("app_test", true).unwrap();
    let img = VipsImage::from_file("kodim01.png").unwrap();
    let img = img.thumbnail(123, 234, VipsSize::VIPS_SIZE_FORCE).unwrap();
    img.write_to_file("kodim01_123x234.png").unwrap();
}
//
//#[test]
//fn unique_instance() {
//    assert!(VipsInstance::new("app_test", true).is_err());
//}
