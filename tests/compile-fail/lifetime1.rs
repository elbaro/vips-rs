extern crate vips;

use vips::VipsBandFormat;
use vips::VipsInstance;
use vips::VipsImage;
use vips::VipsSize;

fn main() {
    let _instance = VipsInstance::new("lifetime_test", true).unwrap();
    let _img: VipsImage = {
        let pixels = vec![0; 256 * 256 * 3];
        VipsImage::from_memory_reference(&pixels, 256, 256, 3, VipsBandFormat::VIPS_FORMAT_UCHAR).unwrap()
        //~^ ERROR `pixels` does not live long enough
    };
}


