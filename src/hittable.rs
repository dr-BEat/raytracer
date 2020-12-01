use crate::alias::*;
use crate::material::*;
use crate::ray::*;

pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new(r: &Ray, p: Point, t: f64, outward_normal: Vector, material: Material) -> Self {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        Self {
            p: p,
            t: t,
            front_face: front_face,
            normal: if front_face {
                outward_normal
            } else {
                -outward_normal
            },
            material: material,
        }
    }
}

#[derive(Clone)]
pub struct Sphere {
    center: Point,
    radius: f64,
    material: Material,
}

#[derive(Clone)]
pub enum Hittable {
    Sphere(Sphere),
    List(Vec<Hittable>),
}

impl Hittable {
    pub fn new_sphere(center: Point, radius: f64, material: Material) -> Self {
        Self::Sphere(Sphere {
            center: center,
            radius: radius,
            material: material,
        })
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match *self {
            Hittable::Sphere(ref sphere) => {
                let oc = r.origin - sphere.center;
                let a = r.direction.sqrlen();
                let half_b = oc.dot(r.direction);
                let c = oc.sqrlen() - sphere.radius * sphere.radius;

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
                let outward_normal = (p - sphere.center) / sphere.radius;
                Some(HitRecord::new(r, p, root, outward_normal, sphere.material))
            }
            Hittable::List(ref list) => {
                let mut record = None;
                let mut closest_so_far = t_max;
                for hittable in list {
                    if let Some(new_record) = hittable.hit(r, t_min, closest_so_far) {
                        if new_record.t < closest_so_far {
                            closest_so_far = new_record.t;
                            record = Some(new_record);
                        }
                    }
                }
                record
            }
        }
    }
}
