use crate::aabb::AxisAlignedBoundingBox;
use crate::alias::*;
use crate::material::*;
use crate::ray::*;
use rand::prelude::*;
use std::mem;

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
pub struct MovingSphere {
    center_start: Point,
    center_end: Point,
    time_start: f64,
    time_end: f64,
    radius: f64,
    material: Material,
}

impl MovingSphere {
    fn center(&self, time: f64) -> Point {
        self.center_start
            + ((time - self.time_start) / (self.time_end - self.time_start))
                * (self.center_end - self.center_start)
    }
}

#[derive(Clone)]
pub struct BvhNode {
    left: Box<Hittable>,
    right: Box<Hittable>,
    bounding_box: AxisAlignedBoundingBox,
}

#[derive(Clone)]
pub enum Hittable {
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    List(Vec<Hittable>),
    Bvh(BvhNode),
    Empty,
}

impl Hittable {
    pub fn new_sphere(center: Point, radius: f64, material: Material) -> Self {
        Self::Sphere(Sphere {
            center: center,
            radius: radius,
            material: material,
        })
    }

    pub fn new_moving_sphere(
        center_start: Point,
        center_end: Point,
        time_start: f64,
        time_end: f64,
        radius: f64,
        material: Material,
    ) -> Self {
        Self::MovingSphere(MovingSphere {
            center_start: center_start,
            center_end: center_end,
            time_start: time_start,
            time_end: time_end,
            radius: radius,
            material: material,
        })
    }

    pub fn new_bvh(hittables: &mut [Hittable], time_start: f64, time_end: f64) -> Self {
        if hittables.len() >= 2 {
            let axis = rand::thread_rng().gen_range(0, 3);
            let compare = |a: &Hittable, b: &Hittable| {
                a.bounding_box(time_start, time_end)
                    .unwrap()
                    .compare(&b.bounding_box(time_start, time_end).unwrap(), axis)
            };
            hittables.sort_by(compare);
        }

        match hittables {
            [] => Hittable::Empty,
            [hittable] => mem::replace(hittable, Hittable::Empty),
            [left, right] => Hittable::new_bvh_from_left_right(
                mem::replace(left, Hittable::Empty),
                mem::replace(right, Hittable::Empty),
                time_start,
                time_end,
            ),
            _ => {
                let (left, right) = hittables.split_at_mut(hittables.len() / 2);
                Hittable::new_bvh_from_left_right(
                    Hittable::new_bvh(left, time_start, time_end),
                    Hittable::new_bvh(right, time_start, time_end),
                    time_start,
                    time_end,
                )
            }
        }
    }

    fn new_bvh_from_left_right(
        left: Hittable,
        right: Hittable,
        time_start: f64,
        time_end: f64,
    ) -> Self {
        let bounding_box = left
            .bounding_box(time_start, time_end)
            .unwrap()
            .surrounding_box(&right.bounding_box(time_start, time_end).unwrap());
        Self::Bvh(BvhNode {
            left: Box::new(left),
            right: Box::new(right),
            bounding_box: bounding_box,
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
            Hittable::MovingSphere(ref sphere) => {
                let oc = r.origin - sphere.center(r.time);
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
                let outward_normal = (p - sphere.center(r.time)) / sphere.radius;
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
            Hittable::Bvh(ref node) => {
                if !node.bounding_box.hit(r, t_min, t_max) {
                    return None;
                }

                return match node.left.hit(r, t_min, t_max) {
                    Some(left_record) => Some(match node.right.hit(r, t_min, left_record.t) {
                        Some(right_record) if right_record.t < left_record.t => right_record,
                        _ => left_record,
                    }),
                    _ => node.right.hit(r, t_min, t_max),
                };
            }
            Hittable::Empty => None,
        }
    }

    pub fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox> {
        match *self {
            Hittable::Sphere(ref sphere) => Some(AxisAlignedBoundingBox::new(
                sphere.center - Vector::from_array([sphere.radius.abs(); 3]),
                sphere.center + Vector::from_array([sphere.radius.abs(); 3]),
            )),
            Hittable::MovingSphere(ref sphere) => {
                let box0 = AxisAlignedBoundingBox::new(
                    sphere.center(time_start) - Vector::from_array([sphere.radius.abs(); 3]),
                    sphere.center(time_start) + Vector::from_array([sphere.radius.abs(); 3]),
                );
                let box1 = AxisAlignedBoundingBox::new(
                    sphere.center(time_end) - Vector::from_array([sphere.radius.abs(); 3]),
                    sphere.center(time_end) + Vector::from_array([sphere.radius.abs(); 3]),
                );
                Some(box0.surrounding_box(&box1))
            }
            Hittable::List(ref list) => {
                let mut result: Option<AxisAlignedBoundingBox> = None;
                for item in list {
                    if let Some(item_box) = item.bounding_box(time_start, time_end) {
                        result = Some(if let Some(result_box) = result {
                            result_box.surrounding_box(&item_box)
                        } else {
                            item_box
                        })
                    } else {
                        return None;
                    }
                }
                result
            }
            Hittable::Bvh(ref node) => Some(node.bounding_box.clone()),
            Hittable::Empty => None,
        }
    }
}
