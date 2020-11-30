use crate::alias::*;
use crate::hittable::*;
use crate::material::*;
use crate::ray::*;

pub struct Sphere<'a> {
    center: Point,
    radius: f64,
    material: &'a (dyn Material + Sync),
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point, radius: f64, material: &'a (dyn Material + Sync)) -> Self {
        Self {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Hittable for Sphere<'_> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.sqrlen();
        let half_b = oc.dot(r.direction);
        let c = oc.sqrlen() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        Some(HitRecord::new(r, p, root, outward_normal, self.material))
    }
}
