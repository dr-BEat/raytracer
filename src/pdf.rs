use crate::alias::*;
use crate::Hittable;
use rand::seq::SliceRandom;

#[derive(Clone)]
pub enum PDF<'a> {
    Cosine(ONB),
    Mixture(Vec<PDF<'a>>),
    Hittable(&'a Hittable, Point),
}

impl<'a> PDF<'a> {
    pub fn value(&self, direction: &Vector) -> f64 {
        match *self {
            Self::Cosine(ref uvw) => {
                let cosine = direction.normalize().dot(uvw.w());
                if cosine <= 0.0 {
                    0.0
                } else {
                    cosine / std::f64::consts::PI
                }
            }
            Self::Mixture(ref pdfs) => {
                let probability = 1.0 / pdfs.len() as f64;
                pdfs.iter()
                    .map(|pdf| probability * pdf.value(direction))
                    .sum()
            }
            Self::Hittable(ref hittable, ref origin) => hittable.pdf_value(origin, direction),
        }
    }

    pub fn generate(&self) -> Vector {
        match *self {
            Self::Cosine(ref uvw) => uvw.local(&Vector::random_cosine_direction()),
            Self::Mixture(ref pdfs) => pdfs.choose(&mut rand::thread_rng()).unwrap().generate(),
            Self::Hittable(ref hittable, ref origin) => hittable.random(origin),
        }
    }
}
