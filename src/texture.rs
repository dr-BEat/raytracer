use crate::alias::*;
use image::io::Reader as ImageReader;
use image::ImageResult;
use image::RgbImage;
use std::path::Path;

#[derive(Clone)]
pub enum Texture {
    Solid(Color),
    Checker((Box<Texture>, Box<Texture>)),
    Image(RgbImage),
    Normal,
    UV,
}

impl Texture {
    pub fn new_checker(odd: Texture, even: Texture) -> Self {
        Self::Checker((Box::new(odd), Box::new(even)))
    }

    pub fn new_checker_color(odd: Color, even: Color) -> Self {
        Self::new_checker(Texture::Solid(odd), Texture::Solid(even))
    }

    pub fn new_image<P>(path: P) -> ImageResult<Self>
    where
        P: AsRef<Path>,
    {
        let img = ImageReader::open(path)?.decode()?;
        Ok(Self::Image(img.to_rgb8()))
    }

    pub fn value(&self, u: f64, v: f64, p: &Point, normal: &Vector) -> Color {
        match *self {
            Self::Solid(ref color) => *color,
            Self::Checker((ref odd, ref even)) => {
                let scale = 10.0;
                let sines = (scale * p[0]).sin() * (scale * p[1]).sin() * (scale * p[2]).sin();
                if sines < 0.0 {
                    odd.value(u, v, p, normal)
                } else {
                    even.value(u, v, p, normal)
                }
            }
            Self::Image(ref image) => {
                // Clamp input texture coordinates to [0,1] x [1,0]
                let u = u.max(0.0).min(1.0);
                let v = 1.0 - v.max(0.0).min(1.0); // Flip V to image coordinates

                let x = (u * image.width() as f64) as u32;
                let y = (v * image.height() as f64) as u32;

                // Clamp integer mapping, since actual coordinates should be less than 1.0
                let x = x.min(image.width() - 1);
                let y = y.min(image.height() - 1);

                let color_scale = 1.0 / 255.0;
                let pixel = image.get_pixel(x, y);

                Color::from(
                    pixel[0] as f64 * color_scale,
                    pixel[1] as f64 * color_scale,
                    pixel[2] as f64 * color_scale,
                )
            }
            Self::Normal => normal.map(|i| i.abs()),
            Self::UV => Vector::from(u, v, 0.0),
        }
    }
}
