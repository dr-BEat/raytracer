use crate::alias::*;
use crate::hittable::*;
use crate::ray::*;
use crate::texture::*;

#[derive(Clone)]
pub struct Dielectric {
    pub ir: f64, // Index of Refraction
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

#[derive(Clone)]
pub enum Material {
    Lambertian(Texture),
    Dielectric(Dielectric),
    Metal(Metal),
    DiffuseLight(Texture),
    Isotropic(Texture),
}

impl Material {
    pub fn new_lambertian(albedo: Color) -> Self {
        Material::Lambertian(Texture::Solid(albedo))
    }

    pub fn new_lambertian_with_texture(texture: Texture) -> Self {
        Material::Lambertian(texture)
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

    pub fn new_diffuse_light(emit: Color) -> Self {
        Self::DiffuseLight(Texture::Solid(emit))
    }

    pub fn new_isotropic(albedo: Color) -> Self {
        Self::Isotropic(Texture::Solid(albedo))
    }

    pub fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        match *self {
            Self::Lambertian(ref texture) => {
                let mut scatter_direction = hit.normal + random_unit_vector();

                if near_zero(&scatter_direction) {
                    scatter_direction = hit.normal;
                }

                Some((
                    texture.value(hit.u, hit.v, &hit.p),
                    Ray::new(hit.p, scatter_direction, r.time),
                ))
            }
            Self::Dielectric(ref dielectric) => {
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
                    Ray::new(hit.p, direction, r.time),
                ))
            }
            Self::Metal(ref metal) => {
                let reflected = reflect(&r.direction.normalize(), &hit.normal);
                let direction = reflected + metal.fuzz * random_in_unit_sphere();
                if direction.dot(hit.normal) <= 0.0 {
                    return None;
                }

                Some((metal.albedo, Ray::new(hit.p, direction, r.time)))
            }
            Self::DiffuseLight(_) => None,
            Self::Isotropic(ref texture) => Some((
                texture.value(hit.u, hit.v, &hit.p),
                Ray::new(hit.p, random_in_unit_sphere(), r.time),
            )),
        }
    }

    pub fn emit(&self, u: f64, v: f64, p: &Point) -> Color {
        match *self {
            Self::DiffuseLight(ref texture) => texture.value(u, v, p),
            _ => Color::new(),
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}
