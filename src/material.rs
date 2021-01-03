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
        Material::Dielectric(Dielectric { ir })
    }

    pub fn new_metal(albedo: Color, fuzz: f64) -> Self {
        Material::Metal(Metal { albedo, fuzz })
    }

    pub fn new_diffuse_light(emit: Color) -> Self {
        Self::DiffuseLight(Texture::Solid(emit))
    }

    pub fn new_isotropic(albedo: Color) -> Self {
        Self::Isotropic(Texture::Solid(albedo))
    }

    pub fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<(Color, Ray, f64)> {
        match *self {
            Self::Lambertian(ref texture) => {
                let uvw = ONB::from_w(&hit.normal);
                let scatter_direction = uvw.local(&Vector::random_cosine_direction());

                let albedo = texture.value(&hit.uv, &hit.p, &hit.normal);
                let scattered = Ray::new(hit.p, scatter_direction.normalize(), r.time);
                let pdf = hit.normal.dot(scattered.direction) / std::f64::consts::PI;
                Some((albedo, scattered, pdf))
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
                    unit_direction.reflect(&hit.normal)
                } else {
                    unit_direction.refract(&hit.normal, refraction_ratio)
                };
                Some((
                    Color::from(1.0, 1.0, 1.0),
                    Ray::new(hit.p, direction, r.time),
                    1.0,
                ))
            }
            Self::Metal(ref metal) => {
                let reflected = r.direction.normalize().reflect(&hit.normal);
                let direction = reflected + metal.fuzz * Point::random_in_unit_sphere();
                if direction.dot(hit.normal) <= 0.0 {
                    return None;
                }

                Some((metal.albedo, Ray::new(hit.p, direction, r.time), 1.0))
            }
            Self::DiffuseLight(_) => None,
            Self::Isotropic(ref texture) => Some((
                texture.value(&hit.uv, &hit.p, &hit.normal),
                Ray::new(hit.p, Point::random_in_unit_sphere(), r.time),
                1.0,
            )),
        }
    }

    pub fn scattering_pdf(&self, r: &Ray, hit: &HitRecord, scattered: &Ray) -> f64 {
        match *self {
            Self::Lambertian(_) => {
                let cosine = hit.normal.dot(scattered.direction.normalize());
                if cosine <= 0.0 {
                    0.0
                } else {
                    cosine / std::f64::consts::PI
                }
            }
            _ => 1.0,
        }
    }

    pub fn emit(&self, hit: &HitRecord) -> Color {
        match *self {
            Self::DiffuseLight(ref texture) if hit.front_face => {
                texture.value(&hit.uv, &hit.p, &hit.normal)
            }
            _ => Color::new(),
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
