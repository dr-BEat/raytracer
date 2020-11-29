use crate::alias::*;
use crate::hittable::*;
use crate::material::*;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo: albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit.normal + random_unit_vector();

        if near_zero(&scatter_direction) {
            scatter_direction = hit.normal;
        }

        Some((self.albedo, Ray::new(hit.p, scatter_direction)))
    }
}
