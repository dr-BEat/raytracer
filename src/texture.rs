use crate::alias::*;

#[derive(Clone)]
pub enum Texture {
    Solid(Color),
    Checker((Box<Texture>, Box<Texture>)),
}

impl Texture {
    pub fn new_checker(odd: Texture, even: Texture) -> Self {
        Self::Checker((Box::new(odd), Box::new(even)))
    }

    pub fn new_checker_color(odd: Color, even: Color) -> Self {
        Self::new_checker(Texture::Solid(odd), Texture::Solid(even))
    }

    pub fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        match *self {
            Texture::Solid(ref color) => *color,
            Texture::Checker((ref odd, ref even)) => {
                let sines = (10.0 * p[0]).sin() * (10.0 * p[1]).sin() * (10.0 * p[2]).sin();
                if sines < 0.0 {
                    odd.value(u, v, p)
                } else {
                    even.value(u, v, p)
                }
            }
        }
    }
}
