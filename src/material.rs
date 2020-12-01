use crate::alias::*;
use crate::hittable::*;
use crate::ray::*;

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub ir: f64, // Index of Refraction
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Dielectric(Dielectric),
    Metal(Metal),
}

impl Material {
    pub fn new_lambertian(albedo: Color) -> Self {
        Material::Lambertian(Lambertian { albedo: albedo })
    }
    pub fn new_dielectric(ir: f64) -> Self {
        Material::Dielectric(Dielectric { ir: ir })
    }
    pub fn new_metal(albedo: Color, fuzz: f64) -> Self {
        Material::Metal(Metal {
            albedo: albedo,
            fuzz: fuzz,
        })
    }

    pub fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        match *self {
            Material::Lambertian(ref lambertian) => {
                let mut scatter_direction = hit.normal + random_unit_vector();

                if near_zero(&scatter_direction) {
                    scatter_direction = hit.normal;
                }

                Some((lambertian.albedo, Ray::new(hit.p, scatter_direction)))
            }
            Material::Dielectric(ref dielectric) => {
                let refraction_ratio = if hit.front_face {
                    1.0 / dielectric.ir
                } else {
                    dielectric.ir
                };
                let unit_direction = r.direction.normalize();
                let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction = if cannot_refract
                    || reflectance(cos_theta, refraction_ratio) > rand::random()
                {
                    reflect(&unit_direction, &hit.normal)
                } else {
                    refract(&unit_direction, &hit.normal, refraction_ratio)
                };
                Some((
                    Color::from_array([1.0, 1.0, 1.0]),
                    Ray::new(hit.p, direction),
                ))
            }
            Material::Metal(ref metal) => {
                let reflected = reflect(&r.direction.normalize(), &hit.normal);
                let direction = reflected + metal.fuzz * random_in_unit_sphere();
                if direction.dot(hit.normal) <= 0.0 {
                    return None;
                }

                Some((metal.albedo, Ray::new(hit.p, direction)))
            }
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}
