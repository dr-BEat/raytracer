use crate::alias::*;
use crate::hittable::*;
use crate::material::*;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo: albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&r.direction.normalize(), &hit.normal);
        let direction = reflected + self.fuzz * random_in_unit_sphere();
        if direction.dot(hit.normal) <= 0.0 {
            return None;
        }

        Some((self.albedo, Ray::new(hit.p, direction)))
    }
}
