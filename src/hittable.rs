use crate::alias::*;
use crate::material::*;
use crate::ray::*;

pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vector,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        r: &Ray,
        p: Point,
        t: f64,
        outward_normal: Vector,
        material: &'a dyn Material,
    ) -> Self {
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

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
