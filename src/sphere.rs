use crate::alias::*;
use crate::hittable::*;
use crate::ray::*;

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self {
            center: center,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
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
        Some(HitRecord {
            t: root,
            p: p,
            normal: (p - self.center) / self.radius,
        })
    }
}
