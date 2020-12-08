use crate::aabb::AxisAlignedBoundingBox;
use crate::alias::*;
use crate::material::*;
use crate::ray::*;
use rand::prelude::*;
use std::mem;

pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vector,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        r: &Ray,
        p: Point,
        t: f64,
        u: f64,
        v: f64,
        outward_normal: Vector,
        material: &'a Material,
    ) -> Self {
        let front_face = r.direction.dot(outward_normal) < 0.0;
        Self {
            p: p,
            t: t,
            u: u,
            v: v,
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
pub struct Cube {
    p0: Point,
    p1: Point,
    material: Material,
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
pub struct ConstantMedium {
    boundary: Box<Hittable>,
    neg_inv_density: f64,
    material: Material,
}

#[derive(Clone)]
pub struct BvhNode {
    left: Box<Hittable>,
    right: Box<Hittable>,
    bounding_box: AxisAlignedBoundingBox,
}

#[derive(Clone)]
pub enum Hittable {
    Cube(Cube),
    Sphere(Sphere),
    MovingSphere(MovingSphere),
    ConstantMedium(ConstantMedium),
    List(Vec<Hittable>),
    Bvh(BvhNode),
    Empty,
}

impl Hittable {
    pub fn new_cube(p0: Point, p1: Point, material: Material) -> Self {
        Self::Cube(Cube {
            p0: p0,
            p1: p1,
            material: material,
        })
    }

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

    pub fn new_constant_medium(boundary: Hittable, density: f64, color: Color) -> Self {
        Self::ConstantMedium(ConstantMedium {
            boundary: Box::new(boundary),
            neg_inv_density: -(1.0 / density),
            material: Material::new_isotropic(color),
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
            Self::Cube(ref cube) => {
                let mut t_min = t_min;
                let mut t_max = t_max;
                for i in 0..3 {
                    let inv_d = 1.0 / r.direction[i];
                    // Calculate the time when the ray is in the region for this axis.
                    let mut t0 = (cube.p0[i] - r.origin[i]) * inv_d;
                    let mut t1 = (cube.p1[i] - r.origin[i]) * inv_d;
                    if inv_d < 0.0 {
                        mem::swap(&mut t0, &mut t1);
                    }
                    t_min = t0.max(t_min);
                    t_max = t1.min(t_max);
                    // if t_max ever gets smaller then t_min we do not have a hit.
                    if t_max <= t_min {
                        return None;
                    }
                }
                let t = t_min;
                let p = r.at(t);

                Some(HitRecord::new(
                    r,
                    p,
                    t,
                    0.0,
                    0.0,
                    Vector::new(),
                    &cube.material,
                ))
            }
            Self::Sphere(ref sphere) => {
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
                let (u, v) = get_sphere_uv(&outward_normal);
                Some(HitRecord::new(
                    r,
                    p,
                    root,
                    u,
                    v,
                    outward_normal,
                    &sphere.material,
                ))
            }
            Self::MovingSphere(ref sphere) => {
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
                let (u, v) = get_sphere_uv(&outward_normal);
                Some(HitRecord::new(
                    r,
                    p,
                    root,
                    u,
                    v,
                    outward_normal,
                    &sphere.material,
                ))
            }
            Self::ConstantMedium(ref medium) => {
                // Print occasional samples when debugging. To enable, set enableDebug true.
                let enable_debug = false;
                let debugging = enable_debug && rand::random::<f64>() < 0.00001;

                let hit1 = medium.boundary.hit(r, f64::NEG_INFINITY, f64::INFINITY);
                if hit1.is_none() {
                    return None;
                }
                let mut hit1 = hit1.unwrap();

                let hit2 = medium.boundary.hit(r, hit1.t + 0.0001, f64::INFINITY);
                if hit2.is_none() {
                    return None;
                }
                let mut hit2 = hit2.unwrap();

                if debugging {
                    println!("t_min={}, t_max={}", hit1.t, hit2.t);
                }

                hit1.t = hit1.t.max(t_min);
                hit2.t = hit2.t.min(t_max);

                if hit1.t >= hit2.t {
                    return None;
                }

                hit1.t = hit1.t.max(0.0);

                let ray_length = r.direction.length();
                let distance_inside_boundary = (hit2.t - hit1.t) * ray_length;
                let hit_distance = medium.neg_inv_density * rand::random::<f64>().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = hit1.t + hit_distance / ray_length;
                let p = r.at(t);

                if debugging {
                    println!("hit_distance = {}\nt = {}\np = {}", hit_distance, t, p);
                }

                Some(HitRecord::new(
                    r,
                    p,
                    t,
                    0.0,
                    0.0,
                    Vector::from_array([1.0, 0.0, 0.0]), // arbitrary
                    &medium.material,
                ))
            }
            Self::List(ref list) => {
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
            Self::Bvh(ref node) => {
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
            Self::Empty => None,
        }
    }

    pub fn bounding_box(&self, time_start: f64, time_end: f64) -> Option<AxisAlignedBoundingBox> {
        match *self {
            Self::Cube(ref cube) => Some(AxisAlignedBoundingBox::new(cube.p0, cube.p1)),
            Self::Sphere(ref sphere) => Some(AxisAlignedBoundingBox::new(
                sphere.center - Vector::from_array([sphere.radius.abs(); 3]),
                sphere.center + Vector::from_array([sphere.radius.abs(); 3]),
            )),
            Self::MovingSphere(ref sphere) => {
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
            Self::ConstantMedium(ref medium) => medium.boundary.bounding_box(time_start, time_end),
            Self::List(ref list) => {
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
            Self::Bvh(ref node) => Some(node.bounding_box.clone()),
            Self::Empty => None,
        }
    }
}

/// Calculate the UV coordinates on a sphere.
/// u: returned value [0,1] of angle around the Y axis from X=-1.
/// v: returned value [0,1] of angle from Y=-1 to Y=+1.
///     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
///     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
///     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
///
/// # Arguments
///
/// * `p` - A given point on the sphere of radius one, centered at the origin.
fn get_sphere_uv(p: &Point) -> (f64, f64) {
    let theta = (-p[1]).acos();
    let phi = (-p[2]).atan2(p[0]) + std::f64::consts::PI;

    let u = phi / (2.0 * std::f64::consts::PI);
    let v = theta / std::f64::consts::PI;
    (u, v)
}
