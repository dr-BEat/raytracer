use crate::alias::*;
use crate::hittable::*;
use crate::material::*;

pub struct Dielectric {
    ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir: ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r.direction.normalize();
        let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract {
            reflect(&unit_direction, &hit.normal)
        } else {
            refract(&unit_direction, &hit.normal, refraction_ratio)
        };

        Some((
            Color::from_array([1.0, 1.0, 1.0]),
            Ray::new(hit.p, direction),
        ))
    }
}
