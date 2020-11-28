use crate::alias::*;
use crate::ray::*;

pub struct HitRecord {
    pub p: Point,
    pub normal: Vector,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(r: &Ray, p: Point, t: f64, outward_normal: Vector) -> Self {
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
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
