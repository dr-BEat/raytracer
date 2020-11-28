
#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

const IMAGE_WIDTH:u32 = 256;
const IMAGE_HEIGHT:u32 = 256;

fn main() {
    let mut img = Image::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for (x, y) in img.coordinates() {
        img.set_pixel(x, y, px!(x, y, 200));
    }
    let _ = img.save("img.bmp");
    println!("Created img.bmp");
}
