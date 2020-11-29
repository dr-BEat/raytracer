use crate::alias::*;
use crate::hittable::*;
use crate::ray::*;

mod lambertian;
pub use lambertian::*;

pub trait Material {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}
