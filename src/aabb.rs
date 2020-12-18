use crate::alias::*;
use crate::ray::*;
use std::cmp::Ordering;
use std::mem;

#[derive(Clone)]
pub struct AxisAlignedBoundingBox {
    pub minimum: Point,
    pub maximum: Point,
}

impl AxisAlignedBoundingBox {
    pub fn new(minimum: Point, maximum: Point) -> Self {
        Self { minimum, maximum }
    }

    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / r.direction[i];
            // Calculate the time when the ray is in the region for this axis.
            let mut t0 = (self.minimum[i] - r.origin[i]) * inv_d;
            let mut t1 = (self.maximum[i] - r.origin[i]) * inv_d;
            if inv_d < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            // if t_max ever gets smaller then t_min we do not have a hit.
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn surrounding_box(&self, other: &Self) -> Self {
        let mut minimum = Point::new();
        let mut maximum = Point::new();

        for i in 0..3 {
            minimum[i] = self.minimum[i].min(other.minimum[i]);
            maximum[i] = self.maximum[i].max(other.maximum[i]);
        }
        Self::new(minimum, maximum)
    }

    pub fn compare(&self, other: &Self, axis: usize) -> Ordering {
        self.minimum[axis]
            .partial_cmp(&other.minimum[axis])
            .unwrap()
    }
}
