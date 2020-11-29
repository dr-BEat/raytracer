use crate::alias::*;
use crate::hittable::*;
use crate::material::*;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo: albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&r.direction.normalize(), &hit.normal);

        if reflected.dot(hit.normal) <= 0.0 {
            return None;
        }

        Some((self.albedo, Ray::new(hit.p, reflected)))
    }
}
